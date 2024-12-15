use serde::{Deserialize, Serialize};

use crate::feed::feed_id::FeedId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    FeedScreenDefault,

    FeedScreen {
        feed_id: FeedId,
    },

    IntersectedBottom {
        feed_id: FeedId,
        bottom_feed_index: usize,
    },

    ChangedSlide {
        feed_id: FeedId,
    },
}
