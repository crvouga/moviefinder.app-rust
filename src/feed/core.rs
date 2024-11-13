use super::feed_id::FeedId;
use crate::{
    core::query::{Filter, Op},
    media::{
        genre::genre_id::GenreId,
        media_db::interface::{MediaField, MediaQuery},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Feed {
    pub feed_id: FeedId,
    pub start_index: usize,
    pub genre_ids: Vec<GenreId>,
}

const LIMIT: usize = 3;

impl From<&Feed> for MediaQuery {
    fn from(feed: &Feed) -> MediaQuery {
        let genre_clauses = feed
            .genre_ids
            .iter()
            .map(|genre_id| Filter::Clause(MediaField::GenreId, Op::Eq, genre_id.to_string()))
            .collect();

        MediaQuery {
            offset: feed.start_index,
            limit: LIMIT,
            filter: Filter::And(genre_clauses),
        }
    }
}
