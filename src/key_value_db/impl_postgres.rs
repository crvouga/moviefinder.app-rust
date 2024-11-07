use std::{sync::Arc, vec};

use super::interface::{to_namespaced_key, KeyValueDb};
use crate::{
    core::{
        db_conn_sql::interface::DbConnSql,
        logger::interface::Logger,
        sql::{Sql, SqlPrimitive, SqlVarType},
    },
    log_debug,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct ImplPostgres<T: DbConnSql + 'static> {
    db_conn_sql: Arc<T>,
    namespace: Vec<String>,
    logger: Arc<dyn Logger>,
}

impl<T: DbConnSql + 'static> ImplPostgres<T> {
    pub fn new(logger: Arc<dyn Logger>, db_conn_sql: Arc<T>) -> Self {
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

fn parse_row_json(json_row: String) -> Result<Row, String> {
    serde_json::from_str(&json_row).map_err(|e| e.to_string())
}

#[async_trait]
impl<T: DbConnSql> KeyValueDb for ImplPostgres<T> {
    async fn get(&self, key: &str) -> Result<Option<String>, String> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);

        let mut query = Sql::new("SELECT value FROM key_value WHERE key = :key");
        query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
        );
        let queried = self
            .db_conn_sql
            .query(Box::new(parse_row_json), &query)
            .await?;
        let value = queried.first().and_then(|row| row.value.clone());

        log_debug!(self.logger, "get key={}", key);

        Ok(value)
    }

    async fn put(&self, key: &str, value: String) -> Result<(), String> {
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

        log_debug!(self.logger, "put key={}", key);

        self.db_conn_sql
            .query(Box::new(parse_row_json), &query)
            .await?;
        Ok(())
    }

    async fn zap(&self, key: &str) -> Result<(), String> {
        let namespaced_key = to_namespaced_key(&self.namespace, key);

        let mut query = Sql::new("DELETE FROM key_value WHERE key = :key");

        query.set(
            "key",
            SqlVarType::Primitive(SqlPrimitive::Text(namespaced_key.to_string())),
        );

        self.db_conn_sql
            .query(Box::new(parse_row_json), &query)
            .await?;

        log_debug!(self.logger, "zap key={}", key);

        Ok(())
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        let namespace_new = self
            .namespace
            .iter()
            .chain(namespace.iter())
            .cloned()
            .collect();
        Box::new(ImplPostgres {
            db_conn_sql: self.db_conn_sql.clone(),
            namespace: namespace_new,
            logger: self.logger.clone(),
        })
    }
}
