use std::sync::Arc;

use crate::{
    core::logger::interface::Logger,
    key_value_db::interface::KeyValueDb,
    media::{genre::genre_db::interface::GenreDb, media_db::interface::MediaDb},
    person::person_db::interface::PersonDb,
};

use super::{
    controls,
    feed_db::{self, interface::FeedDb},
    feed_session_mapping_db::{self, interface::FeedSessionMappingDb},
    feed_tag_db,
};

pub struct Ctx {
    pub controls: controls::ctx::Ctx,
    pub feed_db: Arc<dyn FeedDb>,
    pub feed_session_mapping_db: Arc<dyn FeedSessionMappingDb>,
    pub media_db: Arc<dyn MediaDb>,
    pub person_db: Arc<dyn PersonDb>,
}

impl Ctx {
    pub fn new(
        media_db: Arc<dyn MediaDb>,
        person_db: Arc<dyn PersonDb>,
        key_value_db: Arc<dyn KeyValueDb>,
        genre_db: Arc<dyn GenreDb>,
        logger: Arc<dyn Logger>,
    ) -> Ctx {
        let feed_db = Arc::new(feed_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        let feed_tag_db = Arc::new(feed_tag_db::impl_::Impl_::new(
            genre_db.clone(),
            person_db.clone(),
        ));

        let feed_session_mapping_db = Arc::new(
            feed_session_mapping_db::impl_key_value_db::ImplKeyValueDb::new(key_value_db.clone()),
        );

        let controls = controls::ctx::Ctx::new(
            key_value_db.clone(),
            feed_db.clone(),
            feed_tag_db.clone(),
            logger.clone(),
        );

        Self {
            person_db,
            media_db,
            controls,
            feed_db,
            feed_session_mapping_db,
        }
    }
}
