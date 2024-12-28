use serde::{Deserialize, Serialize};

use crate::feed::feed_id::FeedId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    FeedScreenDefault,

    IntersectedBottom { feed_id: FeedId },

    ChangedSlide { feed_id: FeedId },
}
