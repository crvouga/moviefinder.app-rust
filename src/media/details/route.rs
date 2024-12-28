use serde::{Deserialize, Serialize};

use crate::media::{media_::Media, media_id::MediaId};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    MediaDetailsScreen { media_id: MediaId, back_url: String },
}

impl Media {
    pub fn details_route(&self, back_url: String) -> Route {
        Route::MediaDetailsScreen {
            back_url,
            media_id: self.id.clone(),
        }
    }
}
