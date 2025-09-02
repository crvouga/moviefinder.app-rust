use super::interface::DbConnSql;
use crate::core::sql::Sql;
use async_trait::async_trait;
use serde_json::{self, Value};

pub struct ImplNoop {}

impl ImplNoop {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl DbConnSql for ImplNoop {
    async fn query(&self, _sql: &Sql) -> Result<Vec<Value>, crate::core::error::CoreError> {
        Ok(vec![])
    }
}
