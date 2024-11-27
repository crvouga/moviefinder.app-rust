use crate::{account, core::human_friendly_base64, feed, media, ui::resizable_image};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Feed(feed::route::Route),
    Account(account::route::Route),
    Media(media::route::Route),
    Unknown(String),
    Favicon,
    RobotsTxt,
    OutputCss,
}

impl Route {
    pub fn encode(&self) -> String {
        human_friendly_base64::encode(self.clone())
    }

    pub fn decode(encoded: &str) -> Route {
        match encoded {
            "/favicon.ico" => Route::Favicon,
            "/robots.txt" => Route::RobotsTxt,
            "/output.css" => Route::OutputCss,
            _ => {
                let decoded = human_friendly_base64::decode(encoded);

                decoded.unwrap_or(Route::Unknown(encoded.to_string()))
            }
        }
    }
}
