use async_trait::async_trait;

use crate::feed::{feed_::Feed, feed_id::FeedId};

#[async_trait]
pub trait FeedDb: Send + Sync {
    async fn get(&self, feed_id: FeedId) -> Result<Option<Feed>, std::io::Error>;
    async fn put(&self, feed: Feed) -> Result<(), std::io::Error>;
    async fn get_else_default(&self, feed_id: FeedId) -> Feed {
        self.get(feed_id.clone())
            .await
            .unwrap_or(None)
            .unwrap_or_default()
    }
}
