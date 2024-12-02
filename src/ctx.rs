use std::sync::Arc;

use crate::{
    core::{
        db_conn_sql::{self, impl_postgres::ImplPostgres},
        http::client::HttpClient,
        logger::{impl_console::ConsoleLogger, interface::Logger},
        tmdb_api::{self, TmdbApi},
    },
    env::Env,
    feed::{
        self, feed_db::interface::FeedDb, feed_session_mapping_db::interface::FeedSessionMappingDb,
        feed_tag_db::interface::FeedTagDb, feed_tags::form_state_db::FeedTagsFormStateDb,
    },
    key_value_db::{self, interface::KeyValueDb},
    media::{
        genre::genre_db::{self, interface::GenreDb},
        media_db::{self, interface::MediaDb},
    },
    person::person_db::{self, interface::PersonDb},
    user::{
        account::account_db::{self, interface::UserAccountDb},
        login_with_sms::verify_sms::{self, interface::VerifySms},
        profile::profile_db::{self, interface::UserProfileDb},
    },
};

pub struct Ctx {
    pub key_value_db: Arc<dyn KeyValueDb>,
    pub db_conn_sql: Arc<ImplPostgres>,
    pub http_client: Arc<HttpClient>,
    pub media_db: Arc<dyn MediaDb>,
    pub tmdb_api: Arc<TmdbApi>,
    pub genre_db: Arc<dyn GenreDb>,
    pub person_db: Arc<dyn PersonDb>,
    pub logger: Arc<dyn Logger>,
    pub feed_db: Arc<dyn FeedDb>,
    pub feed_tags_db: Arc<dyn FeedTagDb>,
    pub feed_session_mapping_db: Arc<dyn FeedSessionMappingDb>,
    pub feed_tags_form_state_db: Arc<FeedTagsFormStateDb>,
    pub verify_sms: Arc<dyn VerifySms>,
    pub user_account_db: Arc<dyn UserAccountDb>,
    pub user_profile_db: Arc<dyn UserProfileDb>,
}

impl Ctx {
    pub async fn new(env: &Env) -> Result<Ctx, String> {
        let logger = Arc::new(ConsoleLogger::new(vec!["app".to_string()]));

        let http_client =
            Arc::new(HttpClient::new(logger.clone()).simulate_latency(env.simulate_latency));

        let db_conn_sql = Arc::new(
            db_conn_sql::impl_postgres::ImplPostgres::new(logger.noop(), &env.database_url)
                .await?
                .simulate_latency(env.simulate_latency),
        );

        let _key_value_db = Arc::new(key_value_db::impl_cached_postgres::ImplCachedPostgres::new(
            logger.clone(),
            db_conn_sql.clone(),
        ));

        let _key_value_db = Arc::new(key_value_db::impl_postgres::ImplPostgres::new(
            logger.clone(),
            db_conn_sql.clone(),
        ));

        let key_value_db = Arc::new(key_value_db::impl_hash_map::ImplHashMap::new());

        let tmdb_api = Arc::new(tmdb_api::TmdbApi::new(
            http_client.clone(),
            env.tmdb_api_read_access_token.clone(),
        ));

        let media_db = Arc::new(media_db::impl_tmdb::ImplTmdb::new(
            logger.noop(),
            tmdb_api.clone(),
        ));

        let genre_db = Arc::new(genre_db::impl_tmdb::ImplTmdb::new(tmdb_api.clone()));

        let person_db = Arc::new(person_db::impl_tmdb::ImplTmdb::new(
            logger.clone(),
            tmdb_api.clone(),
        ));

        let feed_db = Arc::new(feed::feed_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        let feed_tags_db = Arc::new(feed::feed_tag_db::impl_::Impl_::new(
            genre_db.clone(),
            person_db.clone(),
        ));

        let feed_session_mapping_db = Arc::new(
            feed::feed_session_mapping_db::impl_key_value_db::ImplKeyValueDb::new(
                key_value_db.clone(),
            ),
        );

        let feed_tags_form_state_db =
            Arc::new(feed::feed_tags::form_state_db::FeedTagsFormStateDb::new(
                logger.clone(),
                key_value_db.clone(),
            ));

        let verify_sms = Arc::new(verify_sms::impl_fake::ImplFake::new());

        let user_account_db = Arc::new(account_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        let user_profile_db = Arc::new(profile_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        Ok(Ctx {
            logger,
            http_client,
            key_value_db,
            db_conn_sql,
            tmdb_api,
            genre_db,
            person_db,
            media_db,
            feed_db,
            feed_tags_db,
            feed_session_mapping_db,
            feed_tags_form_state_db,
            verify_sms,
            user_account_db,
            user_profile_db,
        })
    }
}
