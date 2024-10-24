use serde::{Deserialize, Serialize};

use super::feed_id::FeedId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Index,
    LoadMore(FeedId),
    ChangedSlide(FeedId),
}
