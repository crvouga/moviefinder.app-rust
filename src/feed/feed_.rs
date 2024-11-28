use crate::{
    core::query::QueryFilter,
    media::media_db::interface::{MediaQuery, MediaQueryField},
};

use super::{feed_id::FeedId, feed_tag::FeedTag};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Feed {
    pub feed_id: FeedId,
    pub start_index: usize,
    pub tags: Vec<FeedTag>,
}
impl Feed {
    pub fn to_media_query(self: &Self, limit: usize) -> MediaQuery {
        let filters = self
            .clone()
            .tags
            .into_iter()
            .map(|feed_tag| feed_tag.into())
            .collect::<Vec<QueryFilter<MediaQueryField>>>();

        MediaQuery {
            limit,
            offset: self.start_index,
            filter: QueryFilter::And(filters),
        }
    }
}
