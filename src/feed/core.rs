use super::{feed_filter::FeedFilter, feed_id::FeedId};
use crate::{
    core::query::QueryFilter,
    media::media_db::interface::{MediaQuery, MediaQueryField},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Feed {
    pub feed_id: FeedId,
    pub start_index: usize,
    pub filters: Vec<FeedFilter>,
}

const LIMIT: usize = 3;

impl From<&Feed> for MediaQuery {
    fn from(feed: &Feed) -> MediaQuery {
        let filters: Vec<QueryFilter<MediaQueryField>> = feed
            .clone()
            .filters
            .into_iter()
            .map(|feed_filter| feed_filter.into())
            .collect();

        MediaQuery {
            offset: feed.start_index,
            limit: LIMIT,
            filter: QueryFilter::And(filters),
        }
    }
}
