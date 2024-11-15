use std::fmt::Debug;

use crate::core::sql::Sql;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

#[async_trait]

pub trait DbConnSql: Send + Sync {
    async fn query<T, F>(&self, parse_row_json: Box<F>, query: &Sql) -> Result<Vec<T>, String>
    where
        F: Fn(String) -> Result<T, String> + Send + Sync,
        T: DeserializeOwned + Send + Sync + Debug;
}
