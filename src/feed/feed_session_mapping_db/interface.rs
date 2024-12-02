use async_trait::async_trait;

use crate::{
    core::{session::session_id::SessionId, unit_of_work::UnitOfWork},
    feed::feed_id::FeedId,
};

#[async_trait]
pub trait FeedSessionMappingDb: Send + Sync {
    async fn get(&self, session_id: SessionId) -> Result<Option<FeedId>, std::io::Error>;
    async fn put(
        &self,
        uow: UnitOfWork,
        session_id: SessionId,
        feed_id: FeedId,
    ) -> Result<(), std::io::Error>;
}
