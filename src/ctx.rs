use std::sync::Arc;

use crate::{
    core::db_conn_sql::{self, impl_postgres::ImplPostgres},
    feed::{
        self,
        feed_db::{self, interface::FeedDb},
        session_feed_mapping_db::interface::SessionFeedMappingDb,
    },
    key_value_db::{self, interface::KeyValueDb},
    media::{
        media_db::{self, interface::MediaDb},
        tmdb_api,
    },
};

pub struct Ctx {
    #[allow(dead_code)]
    pub key_value_db: Arc<dyn KeyValueDb>,
    #[allow(dead_code)]
    pub db_conn_sql: Arc<ImplPostgres>,
    pub media_db: Box<dyn MediaDb>,
    pub feed_db: Box<dyn FeedDb>,
    pub session_feed_mapping_db: Box<dyn SessionFeedMappingDb>,
    pub tmdb_api: Arc<tmdb_api::TmdbApi>,
}

impl Ctx {
    pub async fn new(
        tmdb_api_read_access_token: String,
        database_url: String,
    ) -> Result<Ctx, String> {
        let db_conn_sql =
            Arc::new(db_conn_sql::impl_postgres::ImplPostgres::new(&database_url).await?);

        let key_value_db = Arc::new(key_value_db::impl_postgres::ImplPostgres::new(Arc::clone(
            &db_conn_sql,
        )));

        let tmdb_api = Arc::new(tmdb_api::TmdbApi::new(tmdb_api_read_access_token.clone()));

        let media_db = Box::new(media_db::impl_tmdb::ImplTmdb::new(tmdb_api.clone()));

        let feed_db = Box::new(feed_db::impl_key_value_db::ImplKeyValueDb::new(
            key_value_db.clone(),
        ));

        let session_feed_mapping_db = Box::new(
            feed::session_feed_mapping_db::impl_key_value_db::ImplKeyValueDb::new(
                key_value_db.clone(),
            ),
        );

        Ok(Ctx {
            db_conn_sql,
            tmdb_api,
            media_db,
            session_feed_mapping_db,
            feed_db,
            key_value_db,
        })
    }
}
