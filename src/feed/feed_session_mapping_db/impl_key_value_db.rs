use crate::{
    core::{session::session_id::SessionId, unit_of_work::UnitOfWork},
    feed::feed_id::FeedId,
    key_value_db::interface::KeyValueDbRef,
};
use async_trait::async_trait;

use super::interface::FeedSessionMappingDb;

pub struct KeyValueDb {
    key_value_db: KeyValueDbRef,
}

impl KeyValueDb {
    pub fn new(key_value_db: KeyValueDbRef) -> Self {
        Self {
            key_value_db: key_value_db
                .child(vec!["session-feed-mapping".to_string()])
                .into(),
        }
    }
}

#[async_trait]
impl FeedSessionMappingDb for KeyValueDb {
    async fn get(&self, session_id: SessionId) -> Result<Option<FeedId>, std::io::Error> {
        match self.key_value_db.get(session_id.as_str()).await {
            Ok(Some(value)) => Ok(Some(FeedId::new(&value))),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn put(
        &self,
        uow: UnitOfWork,
        session_id: SessionId,
        feed_id: FeedId,
    ) -> Result<(), std::io::Error> {
        self.key_value_db
            .put(uow, session_id.as_str(), feed_id.as_str().to_string())
            .await?;
        Ok(())
    }
}
