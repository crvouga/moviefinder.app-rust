use crate::{
    core::{http::request::Request, human_friendly_base64},
    feed, media, user,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    User(user::route::Route),
    Media(media::route::Route),
}

impl Route {
    pub fn url(&self) -> String {
        human_friendly_base64::encode(self)
    }

    pub fn from_url(encoded: &str) -> Option<Route> {
        human_friendly_base64::decode(encoded).ok()
    }
}

impl Request {
    pub fn route(&self) -> Option<Route> {
        Route::from_url(&self.url.path)
    }
}
