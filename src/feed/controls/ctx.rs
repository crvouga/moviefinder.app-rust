use std::sync::Arc;

use crate::{
    core::logger::interface::Logger,
    feed::{feed_db::interface::FeedDb, feed_tag_db::interface::FeedTagDb},
    key_value_db::interface::KeyValueDb,
};

use super::form_state_db::FormStateDb;

pub struct Ctx {
    pub form_state_db: Arc<FormStateDb>,
    pub feed_db: Arc<dyn FeedDb>,
    pub feed_tag_db: Arc<dyn FeedTagDb>,
}

impl Ctx {
    pub fn new(
        key_value_db: Arc<dyn KeyValueDb>,
        feed_db: Arc<dyn FeedDb>,
        feed_tag_db: Arc<dyn FeedTagDb>,
        logger: Arc<dyn Logger>,
    ) -> Ctx {
        Self {
            form_state_db: Arc::new(FormStateDb::new(logger.clone(), key_value_db)),
            feed_db,
            feed_tag_db,
        }
    }
}
