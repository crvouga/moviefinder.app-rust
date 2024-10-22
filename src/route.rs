use crate::{account, core, feed, media};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    Account(account::route::Route),
    Media(media::route::Route),
    Favicon,
    RobotsTxt,
    Unknown(String),
}

impl Route {
    pub fn encode(&self) -> String {
        core::route::encode(self.clone())
    }

    pub fn decode(encoded: &String) -> Route {
        match encoded.as_str() {
            "/favicon.ico" => Route::Favicon,
            "/robots.txt" => Route::RobotsTxt,
            _ => {
                let decoded = core::route::decode(encoded);

                decoded.unwrap_or(Route::Unknown(encoded.clone()))
            }
        }
    }
}
