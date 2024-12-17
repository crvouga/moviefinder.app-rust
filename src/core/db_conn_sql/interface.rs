use crate::core::sql::Sql;
use async_trait::async_trait;
use serde_json::Value;
use std::io::Error;
use std::sync::Arc;

#[async_trait]
pub trait DbConnSql: Send + Sync {
    async fn query(&self, query: &Sql) -> Result<Vec<Value>, Error>;
}

pub type DbConnSqlDyn = Arc<dyn DbConnSql>;
