use async_trait::async_trait;

#[allow(dead_code)]
#[async_trait]

pub trait DbConnSql: Send + Sync {
    async fn query<T, F>(&self, parse_row_json: F, query: &str) -> Result<Vec<T>, String>
    where
        F: Fn(String) -> Result<T, String> + Send + Sync;
}
