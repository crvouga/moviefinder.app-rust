use super::feed_id::FeedId;
use crate::{
    core::query::Query,
    media::{genre::genre_id::GenreId, media_db::interface::MediaField},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Feed {
    pub feed_id: FeedId,
    pub active_index: usize,
    pub query: Query<MediaField>,
    pub genre_ids: Vec<GenreId>,
}
