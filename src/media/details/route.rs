use serde::{Deserialize, Serialize};

use crate::media::{media_::Media, media_id::MediaId};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    MediaDetailsScreen { media_id: MediaId },
}

impl Media {
    pub fn details_route(&self) -> Route {
        Route::MediaDetailsScreen {
            media_id: self.id.clone(),
        }
    }
}
