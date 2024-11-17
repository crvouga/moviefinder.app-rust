use async_trait::async_trait;

use crate::{feed::feed_id::FeedId, user_session::session_id::SessionId};

#[async_trait]
pub trait FeedSessionMappingDb: Send + Sync {
    async fn get(&self, session_id: SessionId) -> Result<Option<FeedId>, String>;
    async fn put(&self, session_id: SessionId, feed_id: FeedId) -> Result<(), String>;
}
