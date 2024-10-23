use core::fmt;
use std::fmt::{Display, Formatter};

pub struct QueryParams {
    params: Vec<(String, String)>,
}

impl QueryParams {
    #[allow(dead_code)]
    pub fn empty() -> QueryParams {
        QueryParams { params: vec![] }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, key: &str, value: &str) {
        self.params.push((key.to_string(), value.to_string()));
    }
}

impl Display for QueryParams {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let str = self
            .params
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&");

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let mut start = QueryParams::empty();
        start.add("key", "value");
        start.add("key2", "value2");
        assert_eq!(start.to_string(), "key=value&key2=value2");
    }
}
