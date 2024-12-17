#![allow(dead_code)]
use crate::{
    core::{
        db_conn_sql::{self, interface::DbConnSqlDyn},
        http::{self, client::HttpClientDyn},
        key_value_db::{
            self,
            interface::{KeyValueDb, KeyValueDbDyn},
        },
        logger::{
            impl_console::ConsoleLogger,
            interface::{Logger, LoggerDyn},
        },
        phone_number::{self, country_code::country_code_db::interface::PhoneNumberCountryCodeDb},
        tmdb_api::{self, TmdbApi},
        twilio_api::TwilioApi,
    },
    env::Env,
    feed::{
        self, feed_db::interface::FeedDb, feed_session_mapping_db::interface::FeedSessionMappingDb,
        feed_tag_db::interface::FeedTagDb, feed_tags_form::form_state_db::FeedTagsFormStateDb,
    },
    info,
    media::{
        self,
        genre::genre_db::{self, interface::MediaGenreDb},
        interaction::interaction_db::interface::MediaInteractionDb,
        media_db::{self, interface::MediaDb},
        person::person_db::{self, interface::MediaPersonDb},
    },
    user::{
        login_with_sms::verify_sms::{self, interface::VerifySms},
        user_account::user_account_db::{self, interface::UserAccountDb},
        user_profile::user_profile_db::{self, interface::UserProfileDb},
        user_session::{self, user_session_db::interface::UserSessionDb},
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Ctx {
    pub key_value_db: KeyValueDbDyn,
    pub db_conn_sql: DbConnSqlDyn,
    pub http_client: HttpClientDyn,
    pub logger: LoggerDyn,
    pub tmdb_api: Arc<TmdbApi>,
    pub twilio_api: Arc<TwilioApi>,
    pub media_genre_db: Arc<dyn MediaGenreDb>,
    pub media_person_db: Arc<dyn MediaPersonDb>,
    pub media_db: Arc<dyn MediaDb>,
    pub media_interaction_db: Arc<dyn MediaInteractionDb>,
    pub feed_db: Arc<dyn FeedDb>,
    pub feed_tags_db: Arc<dyn FeedTagDb>,
    pub feed_session_mapping_db: Arc<dyn FeedSessionMappingDb>,
    pub feed_tags_form_state_db: Arc<FeedTagsFormStateDb>,
    pub user_verify_sms: Arc<dyn VerifySms>,
    pub user_account_db: Arc<dyn UserAccountDb>,
    pub user_profile_db: Arc<dyn UserProfileDb>,
    pub user_session_db: Arc<dyn UserSessionDb>,
    pub phone_number_country_code_db: Arc<dyn PhoneNumberCountryCodeDb>,
}

impl Ctx {
    pub async fn new(env: &Env) -> Ctx {
        let logger = Arc::new(ConsoleLogger::new(vec!["app".to_string()]));

        info!(logger, "env stage: {:?}", env.stage);

        let http_client = Arc::new(
            http::client::impl_reqwest::ImplReqwest::new(logger.clone())
                .simulate_latency(env.simulate_latency),
        );

        let db_conn_sql = Arc::new(
            db_conn_sql::impl_postgres::Postgres::new(logger.noop(), &env.database_url)
                .await
                .unwrap()
                .simulate_latency(env.simulate_latency),
        );

        #[derive(Debug)]
        enum KeyValueDbImpl {
            Postgres,
            HashMap,
            CachedPostgres,
        }

        let key_value_db_impl = KeyValueDbImpl::Postgres;

        info!(logger, "key value db impl: {:?}", key_value_db_impl);

        let key_value_db: Arc<dyn KeyValueDb> = match key_value_db_impl {
            KeyValueDbImpl::CachedPostgres => {
                Arc::new(key_value_db::impl_cached_postgres::CachedPostgres::new(
                    logger.clone(),
                    db_conn_sql.clone(),
                ))
            }
            KeyValueDbImpl::Postgres => Arc::new(key_value_db::impl_postgres::Postgres::new(
                logger.clone(),
                db_conn_sql.clone(),
            )),
            KeyValueDbImpl::HashMap => Arc::new(key_value_db::impl_hash_map::HashMap::new()),
        };

        let tmdb_api = Arc::new(tmdb_api::TmdbApi::new(
            http_client.clone(),
            env.tmdb_api_read_access_token.clone(),
        ));

        let media_db = Arc::new(media_db::impl_tmdb::Tmdb::new(
            logger.noop(),
            tmdb_api.clone(),
        ));

        let media_interaction_db = Arc::new(
            media::interaction::interaction_db::impl_postgres::Postgres::new(db_conn_sql.clone()),
        );

        let genre_db = Arc::new(genre_db::impl_tmdb::Tmdb::new(tmdb_api.clone()));

        let person_db = Arc::new(person_db::impl_tmdb::Tmdb::new(
            logger.clone(),
            tmdb_api.clone(),
        ));

        let feed_db = Arc::new(feed::feed_db::impl_key_value_db::KeyValueDb::new(
            key_value_db.clone(),
        ));

        let feed_tags_db = Arc::new(feed::feed_tag_db::impl_poly::Poly::new(
            genre_db.clone(),
            person_db.clone(),
        ));

        let feed_session_mapping_db = Arc::new(
            feed::feed_session_mapping_db::impl_key_value_db::KeyValueDb::new(key_value_db.clone()),
        );

        let feed_tags_form_state_db = Arc::new(
            feed::feed_tags_form::form_state_db::FeedTagsFormStateDb::new(
                logger.clone(),
                key_value_db.clone(),
            ),
        );

        let twilio_api = Arc::new(TwilioApi::new(
            http_client.clone(),
            env.twilio_service_sid.clone(),
            env.twilio_auth_token.clone(),
            env.twilio_account_sid.clone(),
        ));

        #[derive(Debug)]
        enum VerifySmsImpl {
            Twilio,
            Fake,
        }

        let verify_sms_impl = if env.stage.is_local() {
            VerifySmsImpl::Fake
        } else {
            VerifySmsImpl::Twilio
        };

        info!(logger, "verify sms impl: {:?}", verify_sms_impl);

        let verify_sms: Arc<dyn VerifySms> = match verify_sms_impl {
            VerifySmsImpl::Twilio => {
                Arc::new(verify_sms::impl_twilio::Twilio::new(twilio_api.clone()))
            }
            VerifySmsImpl::Fake => Arc::new(verify_sms::impl_fake::Fake::new()),
        };

        let user_account_db = Arc::new(user_account_db::impl_key_value_db::KeyValueDb::new(
            key_value_db.clone(),
        ));

        let user_profile_db = Arc::new(user_profile_db::impl_key_value_db::KeyValueDb::new(
            key_value_db.clone(),
        ));

        let user_session_db = Arc::new(
            user_session::user_session_db::impl_key_value_db::KeyValueDb::new(key_value_db.clone()),
        );

        let phone_number_country_code_db = Arc::new(
            phone_number::country_code::country_code_db::impl_json_file::ImplJsonFile::new(),
        );

        Ctx {
            logger,
            twilio_api,
            http_client,
            key_value_db,
            db_conn_sql,
            tmdb_api,
            media_genre_db: genre_db,
            media_person_db: person_db,
            media_db,
            media_interaction_db,
            feed_db,
            feed_tags_db,
            feed_session_mapping_db,
            feed_tags_form_state_db,
            user_verify_sms: verify_sms,
            user_account_db,
            user_profile_db,
            user_session_db,
            phone_number_country_code_db,
        }
    }
}
