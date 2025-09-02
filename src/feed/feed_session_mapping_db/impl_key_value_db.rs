use crate::{
    core::{
        key_value_db::interface::{KeyValueDbDyn, KeyValueDbExt},
        session::session_id::SessionId,
        unit_of_work::UnitOfWork,
    },
    feed::feed_id::FeedId,
};
use async_trait::async_trait;

use super::interface::FeedSessionMappingDb;

pub struct KeyValueDb {
    key_value_db: KeyValueDbDyn,
}

impl KeyValueDb {
    pub fn new(key_value_db: KeyValueDbDyn) -> Self {
        Self {
            key_value_db: key_value_db
                .namespace(vec!["session-feed-mapping".to_string()])
                .into(),
        }
    }
}

#[async_trait]
impl FeedSessionMappingDb for KeyValueDb {
    async fn get(
        &self,
        session_id: SessionId,
    ) -> Result<Option<FeedId>, crate::core::error::CoreError> {
        match self.key_value_db.get(session_id.as_str()).await {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn put(
        &self,
        uow: UnitOfWork,
        session_id: SessionId,
        feed_id: FeedId,
    ) -> Result<(), crate::core::error::CoreError> {
        self.key_value_db
            .put(uow, session_id.as_str(), &feed_id)
            .await?;
        Ok(())
    }
}
