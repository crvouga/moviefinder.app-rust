use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FormData(HashMap<String, Vec<String>>);

impl FormData {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn empty() -> FormData {
        FormData(HashMap::new())
    }

    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .flat_map(|(key, values)| values.iter().map(move |value| format!("{}={}", key, value)))
            .collect::<Vec<String>>()
            .join("&")
    }

    pub fn get_first(&self, key: &str) -> Option<&String> {
        self.0.get(key).and_then(|values| values.first())
    }

    pub fn get_all(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.0.entry(key).or_insert_with(Vec::new).push(value);
    }
}

impl From<HashMap<String, Vec<String>>> for FormData {
    fn from(form_data: HashMap<String, Vec<String>>) -> FormData {
        FormData(form_data)
    }
}

impl From<&str> for FormData {
    fn from(query_string: &str) -> FormData {
        let mut map = HashMap::new();
        for pair in query_string.split('&') {
            let mut parts = pair.split('=');
            let key = parts.next().unwrap_or("").to_string();
            let value = parts.next().unwrap_or("").to_string();
            map.entry(key).or_insert_with(Vec::new).push(value);
        }
        FormData(map)
    }
}

impl Default for FormData {
    fn default() -> Self {
        FormData(HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() {
        let form_string = "name=John&age=20&hobby=reading&hobby=coding";
        let form_data = FormData::from(form_string);
        let mut expected_data = HashMap::new();
        expected_data.insert("name".to_string(), vec!["John".to_string()]);
        expected_data.insert("age".to_string(), vec!["20".to_string()]);
        expected_data.insert(
            "hobby".to_string(),
            vec!["reading".to_string(), "coding".to_string()],
        );
        let expected = FormData(expected_data);
        assert_eq!(form_data, expected);
    }

    #[test]
    fn test_get_first() {
        let form_string = "name=John&age=20&hobby=reading&hobby=coding";
        let form_data = FormData::from(form_string);
        assert_eq!(form_data.get_first("name"), Some(&"John".to_string()));
        assert_eq!(form_data.get_first("age"), Some(&"20".to_string()));
        assert_eq!(form_data.get_first("hobby"), Some(&"reading".to_string()));
    }

    #[test]
    fn test_get_all() {
        let form_string = "name=John&age=20&hobby=reading&hobby=coding";
        let form_data = FormData::from(form_string);
        assert_eq!(
            form_data.get_all("hobby"),
            Some(&vec!["reading".to_string(), "coding".to_string()])
        );
    }

    #[test]
    fn test_to_string() {
        let form_string = "name=John&age=20&hobby=reading&hobby=coding";
        let form_data = FormData::from(form_string);
        let result_string = form_data.to_string();
        assert!(result_string.contains("name=John"));
        assert!(result_string.contains("age=20"));
        assert!(result_string.contains("hobby=reading"));
        assert!(result_string.contains("hobby=coding"));
    }
}
