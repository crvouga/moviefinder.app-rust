use serde::{Deserialize, Serialize};

use crate::feed::{feed_::Feed, feed_id::FeedId, feed_tag::FeedTag};

#[derive(Default, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct FormState {
    pub feed_id: FeedId,
    pub tags: Vec<FeedTag>,
}

impl FormState {
    pub fn new(feed: &Feed) -> Self {
        Self {
            feed_id: feed.feed_id.clone(),
            tags: feed.tags.clone(),
        }
    }
}
