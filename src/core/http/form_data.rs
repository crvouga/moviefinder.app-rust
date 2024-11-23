use crate::core::params::{Params, ParamsHashMap};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct FormData {
    pub params: ParamsHashMap,
}

impl Params for FormData {
    fn empty() -> Self {
        FormData {
            params: ParamsHashMap::empty(),
        }
    }

    fn from_string(string: &str) -> Self {
        FormData {
            params: ParamsHashMap::from_string(string),
        }
    }

    fn get_all(&self, key: &str) -> Option<&Vec<String>> {
        self.params.get_all(key)
    }

    fn get_first(&self, key: &str) -> Option<&String> {
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
        self.params.to_string()
    }
}
