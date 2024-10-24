use std::sync::Arc;

use crate::{
    feed::feed_db::{self, interface::FeedDb},
    key_value_db::{self, interface::KeyValueDb},
    media::media_db::{self, interface::MediaDb},
};

pub struct Ctx {
    pub media_db: Box<dyn MediaDb>,
    pub key_value_db: Arc<dyn KeyValueDb>,
    pub feed_db: Box<dyn FeedDb>,
}

impl Ctx {
    pub fn new(tmdb_api_read_access_token: String) -> Ctx {
        let media_db = Box::new(media_db::impl_tmdb::Tmdb::new(tmdb_api_read_access_token));

        let key_value_db = Arc::new(key_value_db::impl_hash_map::ImplHashMap::new());

        let feed_db = Box::new(feed_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        Ctx {
            media_db,
            key_value_db,
            feed_db,
        }
    }
}
