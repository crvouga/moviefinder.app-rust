use crate::{account, core, feed, media};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    Account(account::route::Route),
    Media(media::route::Route),
    Unknown(String),
}

impl Route {
    pub fn encode(&self) -> String {
        core::route::encode(self.clone())
    }

    pub fn decode(encoded: &String) -> Route {
        let decoded = core::route::decode(encoded);

        decoded.unwrap_or(Route::Unknown(encoded.clone()))
    }
}
