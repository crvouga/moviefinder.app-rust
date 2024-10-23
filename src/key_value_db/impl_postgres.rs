use async_trait::async_trait;

use crate::core::db_conn_sql::interface::DbConnSql;

use super::interface::KeyValueDb;

pub struct Postgres<T: DbConnSql> {
    db_conn_sql: T,
}

impl<T: DbConnSql> Postgres<T> {
    pub fn new(db_conn_sql: T) -> Self {
        Self { db_conn_sql }
    }
}

#[async_trait]
impl<T: DbConnSql> KeyValueDb for Postgres<T> {
    async fn get(&self, key: &str) -> Result<Option<String>, String> {
        unimplemented!()
    }

    async fn put(&mut self, key: &str, value: String) -> Result<(), String> {
        unimplemented!()
    }

    async fn zap(&mut self, key: &str) -> Result<(), String> {
        unimplemented!()
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        unimplemented!()
    }
}
