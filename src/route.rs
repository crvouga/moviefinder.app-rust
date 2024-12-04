use crate::{
    core::{http::request::Request, human_friendly_base64},
    feed, media,
    ui::route::Routable,
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

impl Routable for Route {
    fn url(&self) -> String {
        human_friendly_base64::encode(self)
    }
}

impl Routable for feed::route::Route {
    fn url(&self) -> String {
        Route::Feed(self.clone()).url()
    }
}

impl Routable for user::route::Route {
    fn url(&self) -> String {
        Route::User(self.clone()).url()
    }
}

impl Routable for media::route::Route {
    fn url(&self) -> String {
        Route::Media(self.clone()).url()
    }
}

impl Request {
    pub fn route(&self) -> Option<Route> {
        Route::from_url(&self.url.path)
    }
}
