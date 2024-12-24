use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

pub fn struct_to_map<T: Serialize>(input: &T) -> BTreeMap<String, String> {
    let json_value = serde_json::to_value(input).expect("Failed to serialize struct");

    let mut map = BTreeMap::new();

    if let Value::Object(obj) = json_value {
        for (key, value) in obj {
            match value {
                Value::String(s) => map.insert(key, s),
                Value::Null => continue,
                _ => map.insert(key, value.to_string()),
            };
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Debug)]
    struct MyStruct {
        field1: String,
        field2: String,
        field3: i32,
    }

    #[test]
    fn test_it_works() {
        let my_struct = MyStruct {
            field1: "Value1".to_string(),
            field2: "Value2".to_string(),
            field3: 123,
        };

        let expected: BTreeMap<String, String> = vec![
            ("field1".to_string(), "Value1".to_string()),
            ("field2".to_string(), "Value2".to_string()),
            ("field3".to_string(), "123".to_string()),
        ]
        .into_iter()
        .collect();

        let actual = struct_to_map(&my_struct);

        assert_eq!(expected, actual);
    }
}
