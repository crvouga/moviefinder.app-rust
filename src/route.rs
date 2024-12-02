use crate::{
    account,
    core::{http::request::Request, human_friendly_base64},
    feed, media,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    Account(account::route::Route),
    Media(media::route::Route),
}

impl Route {
    pub fn encode(&self) -> String {
        human_friendly_base64::encode(self)
    }

    pub fn decode(encoded: &str) -> Option<Route> {
        human_friendly_base64::decode(encoded).ok()
    }
}

impl Request {
    pub fn route(&self) -> Option<Route> {
        Route::decode(&self.url.path)
    }
}
