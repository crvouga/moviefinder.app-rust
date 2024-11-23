use crate::{
    core::session::session_id::SessionId, feed::feed_id::FeedId,
    key_value_db::interface::KeyValueDb,
};
use async_trait::async_trait;
use std::sync::Arc;

use super::interface::FeedSessionMappingDb;

pub struct ImplKeyValueDb {
    key_value_db: Arc<dyn KeyValueDb>,
}

impl ImplKeyValueDb {
    pub fn new(key_value_db: Arc<dyn KeyValueDb>) -> Self {
        Self {
            key_value_db: key_value_db
                .child(vec!["session-feed-mapping".to_string()])
                .into(),
        }
    }
}

#[async_trait]
impl FeedSessionMappingDb for ImplKeyValueDb {
    async fn get(&self, session_id: SessionId) -> Result<Option<FeedId>, String> {
        match self.key_value_db.get(session_id.as_str()).await {
            Ok(Some(value)) => Ok(Some(FeedId::new(&value))),
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
