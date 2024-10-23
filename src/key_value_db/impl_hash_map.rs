use async_trait::async_trait;

use super::interface::KeyValueDb;

#[derive(Clone)]
pub struct HashMap {
    namespace: Vec<String>,
    map: std::collections::HashMap<String, String>,
}

impl HashMap {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            namespace: vec![],
            map: std::collections::HashMap::new(),
        }
    }
    fn to_namespaced_key(&self, key: &str) -> String {
        self.namespace.join(SEPARATOR) + SEPARATOR + key
    }
}

const SEPARATOR: &str = ":";

#[async_trait]
impl KeyValueDb for HashMap {
    async fn get(&self, key: &str) -> Result<Option<String>, String> {
        let namespaced_key = self.to_namespaced_key(key);
        Ok(self.map.get(&namespaced_key).cloned())
    }

    async fn put(&mut self, key: &str, value: String) -> Result<(), String> {
        let namespaced_key = self.to_namespaced_key(key);
        let _ = self.map.insert(namespaced_key.to_string(), value);
        Ok(())
    }

    async fn zap(&mut self, key: &str) -> Result<(), String> {
        let namespaced_key = self.to_namespaced_key(key);
        let _ = self.map.remove(&namespaced_key);
        Ok(())
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        let new_namespace = self
            .namespace
            .iter()
            .chain(namespace.iter())
            .map(|s| s.to_string())
            .collect();

        Box::new(HashMap {
            namespace: new_namespace,
            map: self.map.clone(),
        })
    }
}
