use crate::{account, core::human_friendly_base64, feed, media};
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
        human_friendly_base64::encode(self.clone())
    }

    pub fn decode(encoded: &str) -> Route {
        match encoded {
            "/favicon.ico" => Route::Favicon,
            "/robots.txt" => Route::RobotsTxt,
            _ => {
                let decoded = human_friendly_base64::decode(encoded);

                decoded.unwrap_or(Route::Unknown(encoded.to_string()))
            }
        }
    }
}
