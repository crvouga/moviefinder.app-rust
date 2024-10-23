use async_trait::async_trait;

#[allow(dead_code)]
#[async_trait]
pub trait KeyValueDb: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, String>;
    async fn put(&mut self, key: &str, value: String) -> Result<(), String>;
    async fn zap(&mut self, key: &str) -> Result<(), String>;
    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb>;
}

const SEPARATOR: &str = ":";

pub fn to_namespaced_key(namespace: &[String], key: &str) -> String {
    namespace.join(SEPARATOR) + SEPARATOR + key
}
