use async_trait::async_trait;

use crate::{
    core::{pagination::Paginated, query::Query},
    feed::feed_tag::FeedTag,
};

#[derive(Clone, Debug)]
pub enum FeedTagQueryField {
    Label,
}

#[async_trait]
pub trait FeedTagDb: Send + Sync {
    async fn query(&self, query: Query<FeedTagQueryField>) -> Result<Paginated<FeedTag>, String>;
}
