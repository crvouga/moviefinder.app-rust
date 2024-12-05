use async_trait::async_trait;
use std::sync::Arc;

use crate::core::unit_of_work::UnitOfWork;

use super::interface::KeyValueDb;

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
    async fn get(&self, key: &str) -> Result<Option<String>, std::io::Error> {
        let got_cache = self.cache.get(key).await?;

        if let Some(value) = &got_cache {
            return Ok(Some(value.clone()));
        }

        let got_source = self.source.get(key).await?;

        let uow = UnitOfWork::new();

        if let Some(value) = &got_source {
            self.cache.put(uow.clone(), key, value.clone()).await?;
        }

        Ok(got_source)
    }

    async fn put(&self, uow: UnitOfWork, key: &str, value: String) -> Result<(), std::io::Error> {
        self.source.put(uow.clone(), key, value.clone()).await?;
        self.cache.put(uow, key, value).await?;
        Ok(())
    }

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), std::io::Error> {
        self.source.zap(uow.clone(), key).await?;
        self.cache.zap(uow, key).await?;
        Ok(())
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        Box::new(Self {
            source: self.source.child(namespace.clone()).into(),
            cache: self.cache.child(namespace).into(),
        })
    }
}
