use serde::{Deserialize, Serialize};

use super::{feed_id::FeedId, feed_tags};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    DefaultScreen,

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
