use crate::core::dynamic_data::{DynamicData, DynamicDataBTreeMap};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct JsonData {
    pub params: DynamicDataBTreeMap,
}

impl DynamicData for JsonData {
    fn empty() -> Self {
        JsonData {
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
        let mut new_params = self.clone();
        new_params.params.insert(key, value);
        new_params
    }

    fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    fn to_string(&self) -> String {
        let mut json_map = serde_json::Map::new();
        for (key, values) in &self.params.0 {
            if values.len() == 1 {
                json_map.insert(key.clone(), Value::String(values[0].clone()));
            } else {
                json_map.insert(
                    key.clone(),
                    Value::Array(values.iter().map(|v| Value::String(v.clone())).collect()),
                );
            }
        }
        Value::Object(json_map).to_string()
    }

    fn from_string(string: &str) -> Self {
        let mut map = BTreeMap::new();
        if let Ok(json_value) = serde_json::from_str::<Value>(string) {
            if let Value::Object(obj) = json_value {
                for (key, value) in obj {
                    match value {
                        Value::String(s) => {
                            map.entry(key).or_insert_with(Vec::new).push(s);
                        }
                        Value::Array(arr) => {
                            for item in arr {
                                if let Value::String(s) = item {
                                    map.entry(key.clone()).or_insert_with(Vec::new).push(s);
                                }
                            }
                        }
                        Value::Number(n) => {
                            map.entry(key).or_insert_with(Vec::new).push(n.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }

        JsonData {
            params: DynamicDataBTreeMap(map),
        }
    }
}
