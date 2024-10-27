use crate::{
    core::db_conn_sql,
    ctx::Ctx,
    env::Env,
    feed::{feed_db, session_feed_mapping_db},
    key_value_db,
    media::{genre::genre_db, media_db, tmdb_api},
};
use std::sync::Arc;

pub struct BaseFixture {
    pub ctx: Ctx,
    pub env: Env,
}

impl BaseFixture {
    pub async fn new() -> Self {
        let env = Env::load().unwrap();

        let db_conn_sql = Arc::new(
            db_conn_sql::impl_postgres::ImplPostgres::new(&env.database_url)
                .await
                .unwrap(),
        );

        let key_value_db = Arc::new(key_value_db::impl_hash_map::ImplHashMap::new());

        let tmdb_api = Arc::new(tmdb_api::TmdbApi::new(
            env.tmdb_api_read_access_token.clone(),
        ));

        let media_db = Box::new(media_db::impl_random::Random::new());

        let feed_db: Box<feed_db::impl_key_value_db::ImplKeyValueDb> = Box::new(
            feed_db::impl_key_value_db::ImplKeyValueDb::new(key_value_db.clone()),
        );

        let session_feed_mapping_db = Box::new(
            session_feed_mapping_db::impl_key_value_db::ImplKeyValueDb::new(key_value_db.clone()),
        );

        let genre_db = Box::new(genre_db::impl_tmdb::ImplTmdb::new(tmdb_api.clone()));

        let ctx = Ctx {
            genre_db,
            tmdb_api,
            db_conn_sql,
            key_value_db,
            media_db,
            session_feed_mapping_db,
            feed_db,
        };

        Self { ctx, env }
    }
}
