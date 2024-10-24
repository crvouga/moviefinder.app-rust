use serde::{Deserialize, Serialize};

use super::feed_id::FeedId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Feed {
    pub feed_id: FeedId,
    pub active_index: usize,
}

impl Feed {
    #[allow(dead_code)]

    pub fn random() -> Self {
        Self {
            feed_id: FeedId::new("feed_id".to_string()),
            active_index: 0,
        }
    }
}
