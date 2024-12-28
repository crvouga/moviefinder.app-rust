use super::interface::FeedDb;
use crate::{
    core::{
        key_value_db::interface::{KeyValueDbDyn, KeyValueDbExt},
        unit_of_work::UnitOfWork,
    },
    feed::{feed_::Feed, feed_id::FeedId},
};
use async_trait::async_trait;

pub struct KeyValueDb {
    key_value_db: KeyValueDbDyn,
}

impl KeyValueDb {
    pub fn new(key_value_db: KeyValueDbDyn) -> Self {
        Self {
            key_value_db: key_value_db.namespace(vec!["feed".to_string()]).into(),
        }
    }
}

#[async_trait]
impl FeedDb for KeyValueDb {
    async fn get(&self, feed_id: FeedId) -> Result<Option<Feed>, crate::core::error::Error> {
        self.key_value_db.get(feed_id.as_str()).await
    }

    async fn put(&self, uow: UnitOfWork, feed: Feed) -> Result<(), crate::core::error::Error> {
        self.key_value_db
            .put(uow, feed.feed_id.as_str(), &feed)
            .await
    }
}
