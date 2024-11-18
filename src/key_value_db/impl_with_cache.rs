use super::interface::KeyValueDb;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ImplWithCache {
    source: Arc<dyn KeyValueDb>,
    cache: Arc<dyn KeyValueDb>,
}

impl ImplWithCache {
    pub fn new(source: Arc<dyn KeyValueDb>, cache: Arc<dyn KeyValueDb>) -> Self {
        Self { source, cache }
    }
}

#[async_trait]
impl KeyValueDb for ImplWithCache {
    async fn get(&self, key: &str) -> Result<Option<String>, String> {
        let got_cache = self.cache.get(key).await?;

        if let Some(value) = &got_cache {
            return Ok(Some(value.clone()));
        }

        let got_source = self.source.get(key).await?;

        if let Some(value) = &got_source {
            self.cache.put(key, value.clone()).await?;
        }

        Ok(got_source)
    }

    async fn put(&self, key: &str, value: String) -> Result<(), String> {
        let _fut = self.source.put(key, value.clone());
        self.cache.put(key, value).await
    }

    async fn zap(&self, key: &str) -> Result<(), String> {
        let _fut = self.source.zap(key);
        self.cache.zap(key).await
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        Box::new(Self {
            source: self.source.child(namespace.clone()).into(),
            cache: self.cache.child(namespace).into(),
        })
    }
}
