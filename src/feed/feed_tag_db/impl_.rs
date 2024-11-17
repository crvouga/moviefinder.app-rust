use std::sync::Arc;

use super::interface::{FeedTagDb, FeedTagQueryField};
use crate::{
    core::{pagination::Paginated, query::Query},
    feed::feed_tag::FeedTag,
    media::genre::genre_db::interface::GenreDb,
};
use async_trait::async_trait;

pub struct Impl_ {
    genre_db: Arc<dyn GenreDb>,
}

impl Impl_ {
    pub fn new(genre_db: Arc<dyn GenreDb>) -> Self {
        Self { genre_db }
    }
}

#[async_trait]
impl FeedTagDb for Impl_ {
    async fn query(&self, query: Query<FeedTagQueryField>) -> Result<Paginated<FeedTag>, String> {
        let genres = self.genre_db.get_all().await.unwrap_or(vec![]);

        let feed_tags: Vec<FeedTag> = genres
            .iter()
            .map(|genre| FeedTag::Genre(genre.clone()))
            .collect();

        Ok(Paginated {
            total: feed_tags.len(),
            items: feed_tags,
            limit: query.limit,
            offset: query.offset,
        })
    }
}
