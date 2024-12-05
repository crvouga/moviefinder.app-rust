use super::interface::FeedDb;
use crate::{
    core::unit_of_work::UnitOfWork,
    feed::{feed_::Feed, feed_id::FeedId},
    key_value_db::interface::KeyValueDbRef,
};
use async_trait::async_trait;

pub struct KeyValueDb {
    key_value_db: KeyValueDbRef,
}

impl KeyValueDb {
    pub fn new(key_value_db: KeyValueDbRef) -> Self {
        Self {
            key_value_db: key_value_db.child(vec!["feed".to_string()]).into(),
        }
    }
}

#[async_trait]
impl FeedDb for KeyValueDb {
    async fn get(&self, feed_id: FeedId) -> Result<Option<Feed>, std::io::Error> {
        match self.key_value_db.get(feed_id.as_str()).await {
            Ok(Some(value)) => serde_json::from_str::<Feed>(&value)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                .map(Some),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn put(&self, uow: UnitOfWork, feed: Feed) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string(&feed)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

        self.key_value_db
            .put(uow, feed.feed_id.as_str(), serialized)
            .await?;
        Ok(())
    }
}
