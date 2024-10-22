use async_trait::async_trait;

#[async_trait]
pub trait KeyValueDb: Send + Sync {
    async fn get(&self, key: String) -> Result<Option<String>, String>;
    async fn put(&self, key: String, value: String) -> Result<(), String>;
    async fn zap(&self, key: String) -> Result<(), String>;
}
