use async_trait::async_trait;
use std::sync::Arc;

use super::interface::KeyValueDb;
use crate::core::error::CoreError;
use crate::core::unit_of_work::UnitOfWork;

#[derive(Clone)]
pub struct WithCache {
    source: Arc<dyn KeyValueDb>,
    cache: Arc<dyn KeyValueDb>,
}

impl WithCache {
    pub fn new(source: Arc<dyn KeyValueDb>, cache: Arc<dyn KeyValueDb>) -> Self {
        Self { source, cache }
    }
}

#[async_trait]
impl KeyValueDb for WithCache {
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, CoreError> {
        // 1) check the cache
        if let Some(bytes) = self.cache.get_bytes(key).await? {
            return Ok(Some(bytes));
        }

        // 2) fallback to source
        if let Some(bytes) = self.source.get_bytes(key).await? {
            let uow = UnitOfWork::new();
            // cache it
            self.cache.put_bytes(uow, key, &bytes).await?;
            return Ok(Some(bytes));
        }

        Ok(None)
    }

    async fn put_bytes(&self, uow: UnitOfWork, key: &str, value: &[u8]) -> Result<(), CoreError> {
        self.source.put_bytes(uow.clone(), key, value).await?;
        self.cache.put_bytes(uow, key, value).await?;
        Ok(())
    }

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), CoreError> {
        self.source.zap(uow.clone(), key).await?;
        self.cache.zap(uow, key).await?;
        Ok(())
    }

    fn namespace(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        Box::new(WithCache {
            source: self.source.namespace(namespace.clone()).into(),
            cache: self.cache.namespace(namespace).into(),
        })
    }
}
