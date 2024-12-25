use super::interface::{to_namespaced_key, KeyValueDb};
use crate::core::{error::Error, unit_of_work::UnitOfWork};
use async_trait::async_trait;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct HashMap {
    namespace: Vec<String>,
    // Store bytes instead of strings
    map: Arc<RwLock<std::collections::HashMap<String, Vec<u8>>>>,
}

impl HashMap {
    pub fn new() -> Self {
        Self {
            namespace: vec![],
            map: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    fn to_namespaced_key(&self, key: &str) -> String {
        to_namespaced_key(&self.namespace, key)
    }
}

#[async_trait]
impl KeyValueDb for HashMap {
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let namespaced_key = self.to_namespaced_key(key);
        let map = self.map.read().unwrap();
        Ok(map.get(&namespaced_key).cloned())
    }

    async fn put_bytes(&self, uow: UnitOfWork, key: &str, value: &[u8]) -> Result<(), Error> {
        let namespaced_key = self.to_namespaced_key(key);
        let map_arc = Arc::clone(&self.map);

        let old_value = {
            let mut map = map_arc.write().unwrap();
            // Insert the new value (clone so we own it)
            map.insert(namespaced_key.clone(), value.to_vec())
        };

        // Register rollback closure
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

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), Error> {
        let namespaced_key = self.to_namespaced_key(key);
        let map_arc = Arc::clone(&self.map);

        let removed_value = {
            let mut map = map_arc.write().unwrap();
            map.remove(&namespaced_key)
        };

        // Register rollback closure
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
        let new_namespace: Vec<String> = self
            .namespace
            .iter()
            .chain(namespace.iter())
            .cloned()
            .collect();

        Box::new(HashMap {
            namespace: new_namespace,
            map: Arc::clone(&self.map),
        })
    }
}
