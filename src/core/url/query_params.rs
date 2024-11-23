use crate::core::{
    params::{Params, ParamsHashMap},
    url_encoded,
};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct QueryParams {
    params: ParamsHashMap,
}

impl Params for QueryParams {
    fn empty() -> Self {
        QueryParams {
            params: ParamsHashMap::empty(),
        }
    }

    fn from_string(string: &str) -> Self {
        QueryParams {
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
}
