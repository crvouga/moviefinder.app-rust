use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::core::unit_of_work::UnitOfWork;

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
    async fn get(&self, key: &str) -> Result<Option<String>, std::io::Error> {
        let namespaced_key = self.to_namespaced_key(key);
        let map = self.map.read().unwrap();
        Ok(map.get(&namespaced_key).cloned())
    }

    async fn put(&self, uow: UnitOfWork, key: &str, value: String) -> Result<(), std::io::Error> {
        let namespaced_key = self.to_namespaced_key(key);
        let map_arc = Arc::clone(&self.map);

        let old_value = {
            let mut map = map_arc.write().unwrap();
            map.insert(namespaced_key.clone(), value)
        };

        uow.register_rollback(move || {
            let map_arc = Arc::clone(&map_arc);
            let namespaced_key = namespaced_key.clone();
            async move {
                let mut map = map_arc.write().unwrap();
                if let Some(old_value) = old_value {
                    map.insert(namespaced_key, old_value);
                } else {
                    map.remove(&namespaced_key);
                }
                Ok(())
            }
        })
        .await;

        Ok(())
    }
    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), std::io::Error> {
        let namespaced_key = self.to_namespaced_key(key);
        let map_arc = Arc::clone(&self.map);

        let removed_value = {
            let mut map = map_arc.write().unwrap();
            map.remove(&namespaced_key)
        };

        uow.register_rollback(move || {
            let map_arc = Arc::clone(&map_arc);
            let namespaced_key = namespaced_key.clone();
            async move {
                if let Some(removed_value) = removed_value {
                    let mut map = map_arc.write().unwrap();
                    map.insert(namespaced_key.clone(), removed_value);
                }
                Ok(())
            }
        })
        .await;

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
