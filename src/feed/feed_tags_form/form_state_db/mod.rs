use super::form_state::FormState;
use crate::{
    core::{
        key_value_db::interface::{KeyValueDb, KeyValueDbExt},
        logger::interface::Logger,
        unit_of_work::UnitOfWork,
    },
    debug,
    feed::feed_id::FeedId,
};
use std::sync::Arc;

mod interface_test;

pub struct FeedTagsFormStateDb {
    key_value_db: Box<dyn KeyValueDb>,
    logger: Arc<dyn Logger>,
}

impl FeedTagsFormStateDb {
    pub fn new(logger: Arc<dyn Logger>, key_value_db: Arc<dyn KeyValueDb>) -> Self {
        Self {
            logger: logger.child("form_state_db"),
            key_value_db: key_value_db.namespace(vec![
                "feed".to_string(),
                "controls".to_string(),
                "form-state".to_string(),
            ]),
        }
    }

    pub async fn get(
        &self,
        feed_id: &FeedId,
    ) -> Result<Option<FormState>, crate::core::error::CoreError> {
        debug!(self.logger, "get {:?}", feed_id);
        self.key_value_db.get(feed_id.as_str()).await
    }

    pub async fn put(
        &self,
        uow: UnitOfWork,
        form_state: &FormState,
    ) -> Result<(), crate::core::error::CoreError> {
        debug!(self.logger, "put {:?}", form_state);

        self.key_value_db
            .put(uow, form_state.feed_id.as_str(), &form_state)
            .await
    }
}
