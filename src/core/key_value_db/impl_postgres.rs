use super::interface::{to_namespaced_key, KeyValueDb};
use crate::{
    core::{
        db_conn_sql::{self, interface::DbConnSqlDyn},
        logger::interface::LoggerDyn,
        sql::{Sql, SqlPrimitive, SqlVarType},
        unit_of_work::UnitOfWork,
    },
    debug,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::vec;

pub struct Postgres {
    db_conn_sql: DbConnSqlDyn,
    namespace: Vec<String>,
    logger: LoggerDyn,
}

impl Postgres {
    pub fn new(logger: LoggerDyn, db_conn_sql: DbConnSqlDyn) -> Self {
        Self {
            logger: logger.child("key_value_db_postgres"),
            db_conn_sql,
            namespace: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Row {
    key: Option<String>,
    value: Option<String>,
    created_at_posix: Option<i64>,
    updated_at_posix: Option<i64>,
}

#[async_trait]
impl KeyValueDb for Postgres {
    async fn get(&self, key: &str) -> Result<Option<String>, crate::core::error::Error> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);

        let mut query = Sql::new("SELECT value FROM key_value WHERE key = :key");
        query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
        );
        let queried: Vec<Row> = db_conn_sql::query(self.db_conn_sql.clone(), &query).await?;
        let value = queried.first().and_then(|row| row.value.clone());

        debug!(self.logger, "get key={}", key);

        Ok(value)
    }

    async fn put(
        &self,
        uow: UnitOfWork,
        key: &str,
        value: String,
    ) -> Result<(), crate::core::error::Error> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);

        let mut query = Sql::new(
            r#"
            INSERT INTO key_value (key, value)
            VALUES (:key, :value)
            ON CONFLICT (key) DO UPDATE
            SET value = :value, updated_at_posix = EXTRACT(EPOCH FROM NOW())
            "#,
        );
        query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
        );
        query.set(
            "value",
            SqlVarType::Primitive(SqlPrimitive::Text(value.to_string())),
        );

        debug!(self.logger, "put key={}", key);

        let old_value = db_conn_sql::query::<Row>(self.db_conn_sql.clone(), &query)
            .await?
            .first()
            .and_then(|row| row.value.clone());

        db_conn_sql::query::<Row>(self.db_conn_sql.clone(), &query).await?;

        let db_conn_sql = self.db_conn_sql.clone();
        let namespace = self.namespace.clone();
        let key = key.to_string();
        uow.register_rollback(move || async move {
            match old_value {
                None => {
                    let namespaced_key = to_namespaced_key(&namespace, &key);

                    let mut query = Sql::new("DELETE FROM key_value WHERE key = :key");
                    query.set(
                        "key",
                        SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
                    );

                    db_conn_sql::query::<Row>(db_conn_sql, &query)
                        .await
                        .map(|_| ())
                }

                Some(value) => {
                    let namespaced_key = to_namespaced_key(&namespace, &key);

                    let mut query = Sql::new(
                        r#"
                        INSERT INTO key_value (key, value)
                        VALUES (:key, :value)
                        ON CONFLICT (key) DO UPDATE
                        SET value = :value, updated_at_posix = EXTRACT(EPOCH FROM NOW())
                        "#,
                    );
                    query.set(
                        "key",
                        SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
                    );
                    query.set(
                        "value",
                        SqlVarType::Primitive(SqlPrimitive::Text(value.to_string())),
                    );

                    db_conn_sql::query::<Row>(db_conn_sql, &query)
                        .await
                        .map(|_| ())
                }
            }
        })
        .await;

        Ok(())
    }

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), crate::core::error::Error> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);

        let mut query = Sql::new("DELETE FROM key_value WHERE key = :key");

        query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
        );

        debug!(self.logger, "zap key={}", key);

        let db_conn_sql = self.db_conn_sql.clone();
        let namespace = self.namespace.clone();
        let key = key.to_string();
        let old_value = self.get(&key).await?;

        db_conn_sql::query::<Row>(self.db_conn_sql.clone(), &query).await?;

        uow.register_rollback(move || async move {
            if old_value.is_some() {
                let namespaced_key = to_namespaced_key(&namespace, &key);

                let mut query = Sql::new(
                    r#"
                    INSERT INTO key_value (key, value)
                    VALUES (:key, :value)
                    ON CONFLICT (key) DO UPDATE
                    SET value = :value, updated_at_posix = EXTRACT(EPOCH FROM NOW())
                    "#,
                );
                query.set(
                    "key",
                    SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
                );
                query.set(
                    "value",
                    SqlVarType::Primitive(SqlPrimitive::Text(old_value.unwrap())),
                );

                db_conn_sql::query::<Row>(db_conn_sql, &query)
                    .await
                    .map(|_| ())
            } else {
                Ok(())
            }
        })
        .await;

        Ok(())
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        let namespace_new = self
            .namespace
            .iter()
            .chain(namespace.iter())
            .cloned()
            .collect();
        Box::new(Postgres {
            db_conn_sql: self.db_conn_sql.clone(),
            namespace: namespace_new,
            logger: self.logger.clone(),
        })
    }
}
