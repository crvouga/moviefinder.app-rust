use std::sync::Arc;

use crate::core::unit_of_work::UnitOfWork;
use async_trait::async_trait;

#[async_trait]
pub trait KeyValueDb: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, std::io::Error>;
    async fn put(&self, uow: UnitOfWork, key: &str, value: String) -> Result<(), std::io::Error>;
    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), std::io::Error>;
    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb>;
}

const SEPARATOR: &str = ":";

pub fn to_namespaced_key(namespace: &[String], key: &str) -> String {
    namespace.join(SEPARATOR) + SEPARATOR + key
}

pub type KeyValueDbDyn = Arc<dyn KeyValueDb>;
