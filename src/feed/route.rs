use serde::{Deserialize, Serialize};

use crate::ui::to_url::ToURL;

use super::{feed_id::FeedId, feed_tags};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    ScreenDefault,

    Screen {
        feed_id: FeedId,
    },

    IntersectedBottom {
        feed_id: FeedId,
        bottom_feed_index: usize,
    },

    ChangedSlide {
        feed_id: FeedId,
    },

    Tags(feed_tags::route::Route),
}

impl ToURL for feed_tags::route::Route {
    fn to_url(&self) -> String {
        Route::Tags(self.clone()).to_url()
    }
}
