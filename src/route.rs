use crate::feed::route;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Route {
    Feed(feed::route::Route),
    Account,
    Unknown,
}

pub fn encode(route: Route) -> String {
    let encoded = serde_json::to_string(&route).unwrap_or("".to_owned());
    encoded
}

pub fn decode(encoded: String) -> Route {
    let decoded: Route = serde_json::from_str(&encoded).unwrap_or(Route::Unknown);
    decoded
}
