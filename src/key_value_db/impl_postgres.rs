use std::{sync::Arc, vec};

use super::interface::{to_namespaced_key, KeyValueDb};
use crate::{
    core::{
        db_conn_sql::{self, interface::DbConnSqlRef},
        logger::interface::Logger,
        sql::{Sql, SqlPrimitive, SqlVarType},
        unit_of_work::UnitOfWork,
    },
    debug,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Postgres {
    db_conn_sql: DbConnSqlRef,
    namespace: Vec<String>,
    logger: Arc<dyn Logger>,
}

impl Postgres {
    pub fn new(logger: Arc<dyn Logger>, db_conn_sql: DbConnSqlRef) -> Self {
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

fn parse_row_json(value: Value) -> Result<Row, std::io::Error> {
    serde_json::from_value(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

#[async_trait]
impl KeyValueDb for Postgres {
    async fn get(&self, key: &str) -> Result<Option<String>, std::io::Error> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);

        let mut query = Sql::new("SELECT value FROM key_value WHERE key = :key");
        query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
        );
        let queried = db_conn_sql::query(self.db_conn_sql.clone(), &query, parse_row_json).await?;
        let value = queried.first().and_then(|row| row.value.clone());

        debug!(self.logger, "get key={}", key);

        Ok(value)
    }

    async fn put(&self, uow: UnitOfWork, key: &str, value: String) -> Result<(), std::io::Error> {
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

        let old_value = db_conn_sql::query(self.db_conn_sql.clone(), &query, parse_row_json)
            .await?
            .first()
            .and_then(|row| row.value.clone());

        db_conn_sql::query(self.db_conn_sql.clone(), &query, parse_row_json).await?;

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

                    db_conn_sql::query(db_conn_sql, &query, parse_row_json)
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

                    db_conn_sql::query(db_conn_sql, &query, parse_row_json)
                        .await
                        .map(|_| ())
                }
            }
        })
        .await;

        Ok(())
    }

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), std::io::Error> {
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

        db_conn_sql::query(self.db_conn_sql.clone(), &query, parse_row_json).await?;

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

                db_conn_sql::query(db_conn_sql, &query, parse_row_json)
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
