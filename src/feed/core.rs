use super::feed_id::FeedId;
use crate::media::{genre::genre_id::GenreId, media_db::interface::MediaQuery};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Feed {
    pub feed_id: FeedId,
    pub active_index: usize,
    pub genre_ids: Vec<GenreId>,
    pub query: MediaQuery,
}

impl From<Feed> for MediaQuery {
    fn from(feed: Feed) -> MediaQuery {
        MediaQuery {
            limit: feed.query.limit,
            offset: feed.active_index,
            filter: feed.query.filter,
        }
    }
}
