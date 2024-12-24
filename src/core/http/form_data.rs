use crate::core::dynamic_data::{DynamicData, DynamicDataBTreeMap};
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
pub struct FormData {
    pub params: DynamicDataBTreeMap,
}

impl DynamicData for FormData {
    fn empty() -> Self {
        FormData {
            params: DynamicDataBTreeMap::empty(),
        }
    }

    fn get_all(&self, key: &str) -> Option<&Vec<String>> {
        self.params.get_all(key)
    }

    fn get_first(&self, key: &str) -> Option<String> {
        self.params.get_first(key)
    }

    fn insert(&mut self, key: &str, value: String) -> Self {
        FormData {
            params: self.params.insert(key, value),
        }
    }

    fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    fn to_string(&self) -> String {
        self.params
            .0
            .iter()
            .flat_map(|(key, values)| values.iter().map(move |value| format!("{}={}", key, value)))
            .collect::<Vec<String>>()
            .join("&")
    }

    fn from_string(string: &str) -> Self {
        let mut map = BTreeMap::new();
        for pair in string.split('&') {
            let mut parts = pair.split('=');
            let key: String = parts.next().unwrap_or("").to_string();
            if key.is_empty() {
                continue;
            }
            let value = parts.next().unwrap_or("").to_string();
            map.entry(key).or_insert_with(Vec::new).push(value);
        }

        FormData {
            params: DynamicDataBTreeMap(map),
        }
    }
}
