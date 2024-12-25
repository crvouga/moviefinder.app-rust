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

use crate::core::error::Error;

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
    //
    // 1) get_bytes:
    //
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);

        let mut query = Sql::new("SELECT value FROM key_value WHERE key = :key");
        query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key)),
        );

        let rows: Vec<Row> = db_conn_sql::query(self.db_conn_sql.clone(), &query).await?;
        let value_opt = rows.first().and_then(|row| row.value.clone());

        // Convert the stored string value into bytes
        let bytes = value_opt.map(|s| s.into_bytes());
        debug!(self.logger, "get_bytes key={}", key);

        Ok(bytes)
    }

    //
    // 2) put_bytes:
    //
    async fn put_bytes(&self, uow: UnitOfWork, key: &str, value: &[u8]) -> Result<(), Error> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);
        let new_value_str = String::from_utf8_lossy(value).to_string();

        // Grab the old value (if any) for rollback
        let mut select_query = Sql::new("SELECT value FROM key_value WHERE key = :key");
        select_query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.clone())),
        );
        let old_value = db_conn_sql::query::<Row>(self.db_conn_sql.clone(), &select_query)
            .await?
            .first()
            .and_then(|row| row.value.clone())
            .map(|s| s.into_bytes()); // store old_value as bytes

        // Upsert the new value
        let mut upsert_query = Sql::new(
            r#"
            INSERT INTO key_value (key, value)
            VALUES (:key, :value)
            ON CONFLICT (key) DO UPDATE
            SET value = :value, updated_at_posix = EXTRACT(EPOCH FROM NOW())
            "#,
        );
        upsert_query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.clone())),
        );
        upsert_query.set(
            "value",
            SqlVarType::Primitive(SqlPrimitive::Text(new_value_str)),
        );

        debug!(self.logger, "put_bytes key={}", key);
        db_conn_sql::query::<Row>(self.db_conn_sql.clone(), &upsert_query).await?;

        //
        // Register rollback closure
        //
        let db_conn_sql = self.db_conn_sql.clone();
        let namespace = self.namespace.clone();
        let key = key.to_string();
        uow.register_rollback(move || async move {
            match old_value {
                // If there was no old value, rollback means "delete"
                None => {
                    let namespaced_key = to_namespaced_key(&namespace, &key);
                    let mut delete_query = Sql::new("DELETE FROM key_value WHERE key = :key");
                    delete_query.set(
                        "key",
                        SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key)),
                    );
                    db_conn_sql::query::<Row>(db_conn_sql.clone(), &delete_query)
                        .await
                        .map(|_| ())
                }

                // If there was an old value, rollback means "put it back"
                Some(old_val_bytes) => {
                    let namespaced_key = to_namespaced_key(&namespace, &key);
                    let old_val_str = String::from_utf8_lossy(&old_val_bytes).to_string();

                    let mut upsert_query = Sql::new(
                        r#"
                        INSERT INTO key_value (key, value)
                        VALUES (:key, :value)
                        ON CONFLICT (key) DO UPDATE
                        SET value = :value, updated_at_posix = EXTRACT(EPOCH FROM NOW())
                        "#,
                    );
                    upsert_query.set(
                        "key",
                        SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
                    );
                    upsert_query.set(
                        "value",
                        SqlVarType::Primitive(SqlPrimitive::Text(old_val_str)),
                    );

                    db_conn_sql::query::<Row>(db_conn_sql.clone(), &upsert_query)
                        .await
                        .map(|_| ())
                }
            }
        })
        .await;

        Ok(())
    }

    //
    // 3) zap:
    //
    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), Error> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);

        // Grab the old value for rollback
        let old_value = self.get_bytes(key).await?;

        let mut delete_query = Sql::new("DELETE FROM key_value WHERE key = :key");
        delete_query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.clone())),
        );

        debug!(self.logger, "zap key={}", key);
        db_conn_sql::query::<Row>(self.db_conn_sql.clone(), &delete_query).await?;

        //
        // Register rollback closure
        //
        let db_conn_sql = self.db_conn_sql.clone();
        let namespace = self.namespace.clone();
        let key = key.to_string();
        uow.register_rollback(move || async move {
            if let Some(old_val_bytes) = old_value {
                let namespaced_key = to_namespaced_key(&namespace, &key);
                let old_val_str = String::from_utf8_lossy(&old_val_bytes).to_string();

                let mut upsert_query = Sql::new(
                    r#"
                    INSERT INTO key_value (key, value)
                    VALUES (:key, :value)
                    ON CONFLICT (key) DO UPDATE
                    SET value = :value, updated_at_posix = EXTRACT(EPOCH FROM NOW())
                    "#,
                );
                upsert_query.set(
                    "key",
                    SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
                );
                upsert_query.set(
                    "value",
                    SqlVarType::Primitive(SqlPrimitive::Text(old_val_str)),
                );

                db_conn_sql::query::<Row>(db_conn_sql.clone(), &upsert_query)
                    .await
                    .map(|_| ())
            } else {
                // If there was no old value, nothing to restore
                Ok(())
            }
        })
        .await;

        Ok(())
    }

    //
    // 4) child:
    //
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
