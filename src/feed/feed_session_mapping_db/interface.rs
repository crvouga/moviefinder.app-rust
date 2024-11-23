use async_trait::async_trait;

use crate::{core::session::session_id::SessionId, feed::feed_id::FeedId};

#[async_trait]
pub trait FeedSessionMappingDb: Send + Sync {
    async fn get(&self, session_id: SessionId) -> Result<Option<FeedId>, String>;
    async fn put(&self, session_id: SessionId, feed_id: FeedId) -> Result<(), String>;
}
