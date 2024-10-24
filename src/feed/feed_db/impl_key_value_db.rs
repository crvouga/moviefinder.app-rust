use std::sync::Arc;

use super::interface::FeedDb;
use crate::{
    feed::{core::Feed, feed_id::FeedId},
    key_value_db::interface::KeyValueDb,
};
use async_trait::async_trait;

pub struct ImplKeyValueDb {
    key_value_db: Arc<dyn KeyValueDb>,
}

impl ImplKeyValueDb {
    pub fn new(key_value_db: Arc<dyn KeyValueDb>) -> Self {
        Self { key_value_db }
    }
}

#[async_trait]
impl FeedDb for ImplKeyValueDb {
    async fn get(&self, feed_id: FeedId) -> Result<Option<Feed>, String> {
        match self.key_value_db.get(feed_id.as_str()).await {
            Ok(Some(value)) => serde_json::from_str::<Feed>(&value)
                .map_err(|e| e.to_string())
                .map(Some),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn put(&self, feed: Feed) -> Result<(), String> {
        let serialized = serde_json::to_string(&feed).map_err(|e| e.to_string())?;
        self.key_value_db
            .put(feed.feed_id.as_str(), serialized)
            .await?;
        Ok(())
    }
}
