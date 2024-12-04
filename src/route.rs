use crate::{
    core::{http::request::Request, human_friendly_base64},
    feed, media,
    ui::to_url::ToURL,
    user,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    User(user::route::Route),
    Media(media::route::Route),
}

impl Route {
    pub fn from_url(encoded: &str) -> Option<Route> {
        human_friendly_base64::decode(encoded).ok()
    }
}

impl ToURL for Route {
    fn to_url(&self) -> String {
        human_friendly_base64::encode(self)
    }
}

impl ToURL for feed::route::Route {
    fn to_url(&self) -> String {
        Route::Feed(self.clone()).to_url()
    }
}

impl ToURL for user::route::Route {
    fn to_url(&self) -> String {
        Route::User(self.clone()).to_url()
    }
}

impl ToURL for media::route::Route {
    fn to_url(&self) -> String {
        Route::Media(self.clone()).to_url()
    }
}

impl Request {
    pub fn route(&self) -> Option<Route> {
        Route::from_url(&self.url.path)
    }
}
