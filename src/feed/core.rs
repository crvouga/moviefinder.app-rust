use serde::{Deserialize, Serialize};

use crate::{
    core::query::{Filter, Op, Query},
    media::{genre::genre_id::GenreId, media_db::interface::MediaField},
};

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

impl Into<Query<MediaField>> for Feed {
    fn into(self) -> Query<MediaField> {
        let genre_id_clauses = self
            .genre_ids
            .iter()
            .map(|genre_id| Filter::Clause(MediaField::GenreId, Op::Eq, genre_id.to_string()))
            .collect();

        Query {
            filter: Filter::And(genre_id_clauses),
            limit: 3,
            offset: 0,
        }
    }
}
