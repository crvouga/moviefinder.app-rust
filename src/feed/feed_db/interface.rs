use async_trait::async_trait;

use crate::{
    core::unit_of_work::UnitOfWork,
    feed::{feed_::Feed, feed_id::FeedId},
};

#[async_trait]
pub trait FeedDb: Send + Sync {
    async fn get(&self, feed_id: FeedId) -> Result<Option<Feed>, crate::core::error::CoreError>;
    async fn put(&self, uow: UnitOfWork, feed: Feed) -> Result<(), crate::core::error::CoreError>;
    async fn get_else_default(&self, feed_id: FeedId) -> Feed {
        self.get(feed_id.clone())
            .await
            .unwrap_or(None)
            .unwrap_or_default()
    }
}
