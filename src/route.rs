use crate::{
    account,
    core::{base32, http::request::Request, human_friendly_base64},
    feed, media,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    Account(account::route::Route),
    Media(media::route::Route),
}

fn remove_leading_slash(s: &str) -> &str {
    s.strip_prefix('/').unwrap_or(s)
}

impl Route {
    pub fn encode(&self) -> String {
        // human_friendly_base64::encode(self)
        let json = serde_json::to_string(self).ok().unwrap_or("".to_string());

        base32::encode(base32::Alphabet::Crockford, json.as_bytes())
    }

    pub fn decode(encoded: &str) -> Option<Route> {
        // human_friendly_base64::decode(encoded).ok()

        let encoded = base32::decode(base32::Alphabet::Crockford, remove_leading_slash(encoded))?;

        let json = String::from_utf8(encoded).ok()?;

        serde_json::from_str(&json).ok()
    }
}

impl Request {
    pub fn route(&self) -> Option<Route> {
        Route::decode(&self.url.path)
    }
}
