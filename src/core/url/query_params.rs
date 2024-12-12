use std::collections::HashMap;

use crate::core::{
    dynamic_data::{DynamicData, DynamicDataHashMap},
    url_encoded,
};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct QueryParams {
    pub params: DynamicDataHashMap,
}

impl DynamicData for QueryParams {
    fn empty() -> Self {
        QueryParams {
            params: DynamicDataHashMap::empty(),
        }
    }

    fn get_all(&self, key: &str) -> Option<&Vec<String>> {
        self.params.get_all(key)
    }

    fn get_first(&self, key: &str) -> Option<String> {
        self.params.get_first(key)
    }

    fn insert(&mut self, key: &str, value: String) -> Self {
        QueryParams {
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
            .flat_map(|(key, values)| {
                values
                    .iter()
                    .map(move |value| format!("{}={}", key, url_encoded::encode(value)))
            })
            .collect::<Vec<String>>()
            .join("&")
    }

    fn from_string(string: &str) -> Self {
        let mut map = HashMap::new();
        for pair in string.split('&') {
            let mut parts = pair.split('=');
            let key: String = parts.next().unwrap_or("").to_string();
            if key.is_empty() {
                continue;
            }
            let value = parts.next().unwrap_or("").to_string();
            map.entry(key).or_insert_with(Vec::new).push(value);
        }

        QueryParams {
            params: DynamicDataHashMap(map),
        }
    }
}
