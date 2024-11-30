use serde::{Deserialize, Serialize};

use super::{feed_id::FeedId, feed_tags};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Default,

    Index {
        feed_id: FeedId,
    },

    IntersectedBottom {
        feed_id: FeedId,
        bottom_feed_index: usize,
    },

    ChangedSlide {
        feed_id: FeedId,
    },

    Controls(feed_tags::route::Route),
}
