use crate::{
    core::{db_conn_sql, http::client::HttpClient, logger::impl_console::ConsoleLogger, tmdb_api},
    ctx::Ctx,
    env::Env,
    feed, key_value_db,
    media::{genre::genre_db, media_db},
};
use std::sync::Arc;

pub struct BaseFixture {
    pub ctx: Ctx,
    pub env: Env,
}

impl BaseFixture {
    pub async fn new() -> Self {
        let logger = Arc::new(ConsoleLogger::new(vec!["app".to_string()]));

        let http_client = Arc::new(HttpClient::new(logger.clone()));

        let env = Env::load().unwrap();

        let db_conn_sql = Arc::new(
            db_conn_sql::impl_postgres::ImplPostgres::new(logger.clone(), &env.database_url)
                .await
                .unwrap(),
        );

        let key_value_db = Arc::new(key_value_db::impl_hash_map::ImplHashMap::new());

        let tmdb_api = Arc::new(tmdb_api::TmdbApi::new(
            http_client.clone(),
            env.tmdb_api_read_access_token.clone(),
        ));

        let media_db = Arc::new(media_db::impl_random::Random::new());

        let genre_db = Arc::new(genre_db::impl_tmdb::ImplTmdb::new(tmdb_api.clone()));

        let feed = feed::ctx::Ctx::new(
            media_db.clone(),
            key_value_db.clone(),
            genre_db.clone(),
            logger.clone(),
        );

        let ctx = Ctx {
            feed,
            genre_db,
            tmdb_api,
            db_conn_sql,
            key_value_db,
            media_db,
            logger,
        };

        Self { ctx, env }
    }
}
