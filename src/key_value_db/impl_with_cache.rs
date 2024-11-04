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
        let got = self.cache.get(key).await?;

        if got.is_none() {
            let got = self.source.get(key).await?;
            if let Some(value) = &got {
                self.cache.put(key, value.clone()).await?;
            }
        }

        Ok(got)
    }

    async fn put(&self, key: &str, value: String) -> Result<(), String> {
        self.source.put(key, value.clone()).await?;
        self.cache.put(key, value).await
    }

    async fn zap(&self, key: &str) -> Result<(), String> {
        self.source.zap(key).await?;
        self.cache.zap(key).await
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        Box::new(Self {
            source: self.source.child(namespace.clone()).into(),
            cache: self.cache.child(namespace).into(),
        })
    }
}
