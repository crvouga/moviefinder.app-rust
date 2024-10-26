use std::sync::Arc;

use crate::{
    feed::{
        self,
        feed_db::{self, interface::FeedDb},
        session_feed_mapping_db::interface::SessionFeedMappingDb,
    },
    key_value_db::{self},
    media::media_db::{self, interface::MediaDb},
};

pub struct Ctx {
    #[allow(dead_code)]
    pub key_value_db: Arc<dyn key_value_db::interface::KeyValueDb>,
    pub media_db: Box<dyn MediaDb>,
    pub feed_db: Box<dyn FeedDb>,
    pub session_feed_mapping_db: Box<dyn SessionFeedMappingDb>,
}

impl Ctx {
    pub fn new(tmdb_api_read_access_token: String) -> Ctx {
        let media_db = Box::new(media_db::impl_tmdb::Tmdb::new(tmdb_api_read_access_token));

        let key_value_db = Arc::new(key_value_db::impl_hash_map::ImplHashMap::new());

        let feed_db = Box::new(feed_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        let session_feed_mapping_db = Box::new(
            feed::session_feed_mapping_db::impl_key_value_db::ImplKeyValueDb::new(
                key_value_db.clone(),
            ),
        );

        let key_value_db = Arc::new(key_value_db::impl_hash_map::ImplHashMap::new());

        Ctx {
            media_db,
            session_feed_mapping_db,
            feed_db,
            key_value_db,
        }
    }
}
