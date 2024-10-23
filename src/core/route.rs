use crate::core;
use serde::{Deserialize, Serialize};

const SEPARATOR: &str = "___";

pub fn encode<T: Serialize + Clone>(route: T) -> String {
    let encoded = serde_json::to_string(&route).unwrap_or("".to_owned());

    let base_64_encoded = core::base64::encode(&encoded);

    let human_friendly = to_human_friendly_str(route);

    format!("{}{}{}", human_friendly, SEPARATOR, base_64_encoded)
}

pub fn decode<T: for<'de> Deserialize<'de>>(encoded: &str) -> Result<T, String> {
    let without_slash = remove_leading_slash(encoded);

    let without_query_params = remove_query_params(&without_slash);

    let separated: Vec<&str> = without_query_params.split(SEPARATOR).collect();

    let second = separated.get(1).unwrap_or(&"");

    let decoded_str = core::base64::decode(second)?;

    serde_json::from_str(&decoded_str).map_err(|e| e.to_string())
}

fn remove_query_params(path: &str) -> String {
    let parts: Vec<&str> = path.split('?').collect();
    parts.first().unwrap_or(&"").to_string()
}

fn remove_leading_slash(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix('/') {
        stripped.to_owned()
    } else {
        path.to_owned()
    }
}

pub fn to_human_friendly_str<T: Serialize>(route: T) -> String {
    let serialized = serde_json::to_string(&route).unwrap_or("".to_owned());

    let mut human_friendly = serialized
        .replace(r#"""#, "")
        .replace(":", ".")
        .replace("{", "")
        .replace("}", "")
        .replace(",", ".")
        .replace(" ", "");

    human_friendly.truncate(human_friendly.len().min(100));
    human_friendly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    enum Route {
        Index,
        Child(ChildRoute),
        Unknown,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    enum ChildRoute {
        Index,
    }

    #[test]
    fn test_encode_decode_route() {
        let route = Route::Index;
        let encoded = encode(&route);
        let decoded: Route = decode(&encoded).unwrap();
        assert_eq!(decoded, route);
    }

    #[test]
    fn test_decode_unknown_route() {
        let invalid_base64 = "invalid_data".to_string();
        let decoded: Route = decode(&invalid_base64).unwrap_or(Route::Unknown);
        assert_eq!(decoded, Route::Unknown);
    }

    #[test]
    fn test_encode_then_decode_route() {
        let route = Route::Child(ChildRoute::Index);
        let encoded = encode(&route);
        let decoded: Route = decode(&encoded).unwrap();
        assert_eq!(decoded, route);
    }

    #[test]
    fn test_decode_with_leading_slash() {
        let route = Route::Child(ChildRoute::Index);
        let encoded = encode(&route);
        let encoded_with_leading_slash = format!("/{}", encoded);
        let decoded: Route = decode(&encoded_with_leading_slash).unwrap();
        assert_eq!(decoded, route);
    }

    #[test]
    fn test_to_human_friendly_str() {
        let route = Route::Index;
        assert_eq!(core::route::to_human_friendly_str(route), "Index");
    }

    #[test]
    fn test_to_human_friendly_str_child() {
        let route = Route::Child(ChildRoute::Index);
        assert_eq!(core::route::to_human_friendly_str(route), "Child.Index");
    }
}
