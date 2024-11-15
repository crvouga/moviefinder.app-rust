use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::interface::{to_namespaced_key, KeyValueDb};

#[derive(Clone)]
pub struct ImplHashMap {
    namespace: Vec<String>,
    map: Arc<RwLock<HashMap<String, String>>>,
}

impl ImplHashMap {
    pub fn new() -> Self {
        Self {
            namespace: vec![],
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn to_namespaced_key(&self, key: &str) -> String {
        to_namespaced_key(&self.namespace, key)
    }
}

#[async_trait]
impl KeyValueDb for ImplHashMap {
    async fn get(&self, key: &str) -> Result<Option<String>, String> {
        let namespaced_key = self.to_namespaced_key(key);
        let map = self.map.read().unwrap();
        Ok(map.get(&namespaced_key).cloned())
    }

    async fn put(&self, key: &str, value: String) -> Result<(), String> {
        let namespaced_key = self.to_namespaced_key(key);
        let mut map = self.map.write().unwrap();
        let _ = map.insert(namespaced_key, value);
        Ok(())
    }

    async fn zap(&self, key: &str) -> Result<(), String> {
        let namespaced_key = self.to_namespaced_key(key);
        let mut map = self.map.write().unwrap();
        let _ = map.remove(&namespaced_key);
        Ok(())
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        let new_namespace = self
            .namespace
            .iter()
            .chain(namespace.iter())
            .map(|s| s.to_string())
            .collect();

        Box::new(ImplHashMap {
            namespace: new_namespace,
            map: Arc::clone(&self.map),
        })
    }
}
