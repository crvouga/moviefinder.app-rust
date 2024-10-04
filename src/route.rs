use crate::feed;
use base64::{decode as base64_decode, encode as base64_encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    Account,
    Unknown,
}

pub fn encode(route: Route) -> String {
    let encoded = serde_json::to_string(&route).unwrap_or("".to_owned());

    let base_64_encoded = base64_encode(encoded);

    base_64_encoded
}

pub fn remove_leading_slash(path: &str) -> String {
    if path.starts_with('/') {
        path[1..].to_owned()
    } else {
        path.to_owned()
    }
}

pub fn decode(base_64_encoded: String) -> Route {
    let decoded_bytes = base64_decode(remove_leading_slash(&base_64_encoded)).unwrap_or(vec![]);

    let encoded = String::from_utf8(decoded_bytes).unwrap_or("".to_owned());

    let decoded: Route = serde_json::from_str(&encoded).unwrap_or(Route::Unknown);

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
