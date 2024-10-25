use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    feed::feed_id::FeedId, key_value_db::interface::KeyValueDb, user_session::session_id::SessionId,
};

use super::interface::SessionFeedMappingDb;

pub struct ImplKeyValueDb {
    key_value_db: Arc<dyn KeyValueDb>,
}

impl ImplKeyValueDb {
    pub fn new(key_value_db: Arc<dyn KeyValueDb>) -> Self {
        Self { key_value_db }
    }
}

#[async_trait]
impl SessionFeedMappingDb for ImplKeyValueDb {
    async fn get(&self, session_id: SessionId) -> Result<Option<FeedId>, String> {
        match self.key_value_db.get(session_id.as_str()).await {
            Ok(Some(value)) => Ok(Some(FeedId::new(value))),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn put(&self, session_id: SessionId, feed_id: FeedId) -> Result<(), String> {
        self.key_value_db
            .put(session_id.as_str(), feed_id.as_str().to_string())
            .await?;
        Ok(())
    }
}
