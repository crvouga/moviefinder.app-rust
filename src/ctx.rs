use std::sync::Arc;

use crate::{
    core::{
        db_conn_sql::{self, impl_postgres::ImplPostgres},
        http::client::HttpClient,
        logger::{impl_console::ConsoleLogger, interface::Logger},
        tmdb_api::{self, TmdbApi},
    },
    env::Env,
    feed,
    key_value_db::{self, interface::KeyValueDb},
    media::{
        genre::genre_db::{self, interface::GenreDb},
        media_db::{self, interface::MediaDb},
    },
};

pub struct Ctx {
    pub key_value_db: Arc<dyn KeyValueDb>,
    pub db_conn_sql: Arc<ImplPostgres>,
    pub media_db: Arc<dyn MediaDb>,
    pub tmdb_api: Arc<TmdbApi>,
    pub genre_db: Arc<dyn GenreDb>,
    pub logger: Arc<dyn Logger>,
    pub feed: feed::ctx::Ctx,
}

impl Ctx {
    pub async fn new(env: Env) -> Result<Ctx, String> {
        let logger = Arc::new(ConsoleLogger::new(vec!["app".to_string()]));

        let http_client =
            Arc::new(HttpClient::new(logger.clone()).simulate_latency(env.simulate_latency));

        let db_conn_sql = Arc::new(
            db_conn_sql::impl_postgres::ImplPostgres::new(logger.noop(), &env.database_url)
                .await?
                .simulate_latency(env.simulate_latency),
        );

        let key_value_db = Arc::new(key_value_db::impl_cached_postgres::ImplCachedPostgres::new(
            logger.clone(),
            db_conn_sql.clone(),
        ));

        // let key_value_db = Arc::new(key_value_db::impl_postgres::ImplPostgres::new(
        //     logger.clone(),
        //     db_conn_sql.clone(),
        // ));

        // let key_value_db = Arc::new(key_value_db::impl_hash_map::ImplHashMap::new());

        let tmdb_api = Arc::new(tmdb_api::TmdbApi::new(
            http_client.clone(),
            env.tmdb_api_read_access_token.clone(),
        ));

        let media_db = Arc::new(media_db::impl_tmdb::ImplTmdb::new(
            logger.noop(),
            tmdb_api.clone(),
        ));

        let genre_db = Arc::new(genre_db::impl_tmdb::ImplTmdb::new(tmdb_api.clone()));

        let feed = feed::ctx::Ctx::new(
            media_db.clone(),
            key_value_db.clone(),
            genre_db.clone(),
            logger.clone(),
        );

        Ok(Ctx {
            logger,
            genre_db,
            db_conn_sql,
            tmdb_api,
            media_db,
            key_value_db,
            feed,
        })
    }
}
