use std::sync::Arc;

use crate::{
    core::{
        db_conn_sql::{self, impl_postgres::ImplPostgres},
        http::client::HttpClient,
        logger::{impl_console::ConsoleLogger, interface::Logger},
    },
    env::Env,
    feed::{
        self,
        feed_db::{self, interface::FeedDb},
        session_feed_mapping_db::interface::SessionFeedMappingDb,
    },
    key_value_db::{self, interface::KeyValueDb},
    media::{
        genre::genre_db::{self, interface::GenreDb},
        media_db::{self, interface::MediaDb},
        tmdb_api::{self, TmdbApi},
    },
};

pub struct Ctx {
    pub key_value_db: Arc<dyn KeyValueDb>,

    pub db_conn_sql: Arc<ImplPostgres>,
    pub media_db: Box<dyn MediaDb>,
    pub feed_db: Box<dyn FeedDb>,
    pub session_feed_mapping_db: Box<dyn SessionFeedMappingDb>,

    pub tmdb_api: Arc<TmdbApi>,
    pub genre_db: Box<dyn GenreDb>,

    pub logger: Arc<dyn Logger>,
}

impl Ctx {
    pub async fn new(env: Env) -> Result<Ctx, String> {
        let logger = Arc::new(ConsoleLogger::new(vec!["app".to_string()]));

        let http_client =
            Arc::new(HttpClient::new(logger.clone()).simulate_latency(env.simulate_latency));

        let db_conn_sql = Arc::new(
            db_conn_sql::impl_postgres::ImplPostgres::new(logger.clone(), &env.database_url)
                .await?
                .simulate_latency(env.simulate_latency),
        );

        let key_value_db = Arc::new(key_value_db::impl_cached_postgres::ImplCachedPostgres::new(
            logger.clone(),
            db_conn_sql.clone(),
        ));

        let tmdb_api = Arc::new(tmdb_api::TmdbApi::new(
            http_client.clone(),
            env.tmdb_api_read_access_token.clone(),
        ));

        let media_db = Box::new(media_db::impl_tmdb::ImplTmdb::new(
            logger.noop(),
            tmdb_api.clone(),
        ));

        let feed_db = Box::new(feed_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        let session_feed_mapping_db = Box::new(
            feed::session_feed_mapping_db::impl_key_value_db::ImplKeyValueDb::new(
                key_value_db.clone(),
            ),
        );

        let genre_db = Box::new(genre_db::impl_tmdb::ImplTmdb::new(tmdb_api.clone()));

        Ok(Ctx {
            logger,
            genre_db,
            db_conn_sql,
            tmdb_api,
            media_db,
            session_feed_mapping_db,
            feed_db,
            key_value_db,
        })
    }
}
