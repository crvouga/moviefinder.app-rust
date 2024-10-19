use crate::feed::{self};
use base64::{decode as base64_decode, encode as base64_encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    Account,
    Unknown,
}

const SEPERATOR: &'static str = "___";

pub fn encode(route: Route) -> String {
    let encoded = serde_json::to_string(&route).unwrap_or("".to_owned());

    let base_64_encoded = base64_encode(encoded);

    let human_friendly = to_human_friendly_str(route);

    let joined = format!("{}{}{}", human_friendly, SEPERATOR, base_64_encoded);

    joined
}

pub fn remove_leading_slash(path: &str) -> String {
    if path.starts_with('/') {
        path[1..].to_owned()
    } else {
        path.to_owned()
    }
}

pub fn decode(base_64_encoded: String) -> Route {
    let without_slash = remove_leading_slash(&base_64_encoded);

    let seperated: Vec<&str> = without_slash.split(SEPERATOR).collect();

    let second = seperated.get(1).unwrap_or(&"");

    let decoded_bytes = base64_decode(second).unwrap_or(vec![]);

    let encoded = String::from_utf8(decoded_bytes).unwrap_or("".to_owned());

    let decoded: Route = serde_json::from_str(&encoded).unwrap_or(Route::Unknown);

    decoded
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
    use crate::feed::route;

    #[test]
    fn test_encode_decode_account_route() {
        let account_route = Route::Account;
        let encoded = encode(account_route.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, account_route);
    }

    #[test]
    fn test_decode_unknown_route() {
        let invalid_base64 = "invalid_data".to_string();
        let decoded = decode(invalid_base64);
        assert_eq!(decoded, Route::Unknown);
    }

    #[test]
    fn test_encode_then_decode_feed_route() {
        let feed_route = Route::Feed(feed::route::Route::Index);
        let encoded = encode(feed_route.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, feed_route);
    }

    #[test]
    fn test_decode_with_leading_slash() {
        let feed_route = Route::Feed(feed::route::Route::Index);
        let encoded = encode(feed_route.clone());
        let encoded_with_leading_slash = format!("/{}", encoded);
        let decoded = decode(encoded_with_leading_slash);
        assert_eq!(decoded, feed_route);
    }

    #[test]
    fn test_to_human_friendly_str_account() {
        let account_route = Route::Account;
        assert_eq!(to_human_friendly_str(account_route), "Account");
    }

    #[test]
    fn test_to_human_friendly_str_feed_index() {
        let feed_route = Route::Feed(route::Route::Index);
        assert_eq!(to_human_friendly_str(feed_route), "Feed.Index");
    }
}
