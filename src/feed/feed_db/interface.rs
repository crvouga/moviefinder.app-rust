use async_trait::async_trait;

use crate::feed::{core::Feed, feed_id::FeedId};
#[allow(dead_code)]
#[async_trait]
pub trait FeedDb: Send + Sync {
    async fn get(&self, feed_id: FeedId) -> Result<Option<Feed>, String>;
    async fn put(&self, feed: Feed) -> Result<(), String>;
}
