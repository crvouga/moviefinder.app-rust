use std::collections::BTreeMap;

pub trait DynamicData {
    fn is_empty(&self) -> bool;

    fn empty() -> Self;

    fn to_string(&self) -> String;

    fn from_string(string: &str) -> Self;

    fn get_first(&self, key: &str) -> Option<String>;

    fn get_all(&self, key: &str) -> Option<&Vec<String>>;

    fn insert(&mut self, key: &str, value: String) -> Self;

    fn from_btree_map(map: BTreeMap<String, String>) -> Self
    where
        Self: Sized,
    {
        let mut params = Self::empty();
        for (key, value) in map {
            params.insert(&key, value);
        }
        params
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
pub struct DynamicDataBTreeMap(pub BTreeMap<String, Vec<String>>);

impl From<BTreeMap<String, Vec<String>>> for DynamicDataBTreeMap {
    fn from(map: BTreeMap<String, Vec<String>>) -> Self {
        DynamicDataBTreeMap(map)
    }
}

impl DynamicData for DynamicDataBTreeMap {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn empty() -> DynamicDataBTreeMap {
        DynamicDataBTreeMap(BTreeMap::new())
    }

    fn to_string(&self) -> String {
        self.0
            .iter()
            .flat_map(|(key, values)| values.iter().map(move |value| format!("{}={}", key, value)))
            .collect::<Vec<String>>()
            .join("&")
    }

    fn from_string(string: &str) -> DynamicDataBTreeMap {
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
        DynamicDataBTreeMap(map)
    }

    fn get_first(&self, key: &str) -> Option<String> {
        let first = self
            .0
            .get(key)
            .and_then(|values| values.first())
            .map(|s| s.to_owned());

        first
    }

    fn get_all(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    fn insert(&mut self, key: &str, value: String) -> Self {
        self.0
            .entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value);

        self.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() {
        let form_string = "name=John&age=20&hobby=reading&hobby=coding";
        let form_data = DynamicDataBTreeMap::from_string(form_string);
        let mut expected_data = BTreeMap::new();
        expected_data.insert("name".to_string(), vec!["John".to_string()]);
        expected_data.insert("age".to_string(), vec!["20".to_string()]);
        expected_data.insert(
            "hobby".to_string(),
            vec!["reading".to_string(), "coding".to_string()],
        );
        let expected = DynamicDataBTreeMap(expected_data);
        assert_eq!(form_data, expected);
    }

    #[test]
    fn test_get_first() {
        let form_string = "name=John&age=20&hobby=reading&hobby=coding";
        let form_data = DynamicDataBTreeMap::from_string(form_string);
        assert_eq!(form_data.get_first("name"), Some("John".to_string()));
        assert_eq!(form_data.get_first("age"), Some("20".to_string()));
        assert_eq!(form_data.get_first("hobby"), Some("reading".to_string()));
    }

    #[test]
    fn test_get_all() {
        let form_string = "name=John&age=20&hobby=reading&hobby=coding";
        let form_data = DynamicDataBTreeMap::from_string(form_string);
        assert_eq!(
            form_data.get_all("hobby"),
            Some(&vec!["reading".to_string(), "coding".to_string()])
        );
    }

    #[test]
    fn test_to_string() {
        let form_string = "name=John&age=20&hobby=reading&hobby=coding";
        let form_data = DynamicDataBTreeMap::from_string(form_string);
        let result_string = form_data.to_string();
        assert!(result_string.contains("name=John"));
        assert!(result_string.contains("age=20"));
        assert!(result_string.contains("hobby=reading"));
        assert!(result_string.contains("hobby=coding"));
    }
}
