use crate::feed;
use base64::{decode as base64_decode, encode as base64_encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Route {
    Feed(feed::route::Route),
    Account,
    Unknown,
}

pub fn encode(route: Route) -> String {
    // Serialize the Route enum to a JSON string
    let encoded = serde_json::to_string(&route).unwrap_or("".to_owned());

    // Encode the JSON string into Base64
    let base_64_encoded = base64_encode(encoded);

    base_64_encoded
}

pub fn decode(base_64_encoded: String) -> Route {
    // Decode the Base64 string into a JSON string
    let decoded_bytes = base64_decode(base_64_encoded).unwrap_or(vec![]);
    let encoded = String::from_utf8(decoded_bytes).unwrap_or("".to_owned());

    // Deserialize the JSON string back into the Route enum
    let decoded: Route = serde_json::from_str(&encoded).unwrap_or(Route::Unknown);

    decoded
}
