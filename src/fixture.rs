use std::sync::Arc;

use crate::{ctx::Ctx, feed::feed_db, key_value_db, media::media_db};

pub struct BaseFixture {
    pub ctx: Ctx,
}

impl BaseFixture {
    pub fn new() -> Self {
        let media_db = Box::new(media_db::impl_random::Random::new());

        let key_value_db = Arc::new(key_value_db::impl_hash_map::ImplHashMap::new());

        let feed_db = Box::new(feed_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        let ctx = Ctx {
            media_db,
            key_value_db,
            feed_db,
        };

        Self { ctx }
    }
}
