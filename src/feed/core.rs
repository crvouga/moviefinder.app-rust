use serde::{Deserialize, Serialize};

use crate::media::genre::genre_id::GenreId;

use super::feed_id::FeedId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Feed {
    pub feed_id: FeedId,
    pub active_index: usize,
    pub genre_ids: Vec<GenreId>,
}

impl Feed {
    #[allow(dead_code)]

    pub fn random() -> Self {
        Self {
            feed_id: FeedId::new("feed_id".to_string()),
            active_index: 0,
            genre_ids: vec![],
        }
    }
}
