use super::interface::{MediaDb, MediaQuery};
use crate::{
    core::{
        cache_db::interface::CacheDbDyn, logger::interface::LoggerDyn, pagination::Paginated,
        tmdb_api::TmdbApi,
    },
    info,
    media::media_::Media,
};
use async_trait::async_trait;
use std::sync::Arc;
use tmdb_query_plan::TmdbQueryPlan;

pub mod tmdb_query_plan;
pub mod tmdb_query_plan_item;
pub struct Tmdb {
    tmdb_api: Arc<TmdbApi>,
    cache_db: CacheDbDyn,
    logger: LoggerDyn,
}

impl Tmdb {
    pub fn new(logger: LoggerDyn, tmdb_api: Arc<TmdbApi>, cache_db: CacheDbDyn) -> Tmdb {
        let logger = logger.child("impl_tmdb");
        Tmdb {
            tmdb_api,
            logger,
            cache_db: cache_db
                .namespace(vec!["media_db".to_string(), "tmdb_api".to_string()])
                .into(),
        }
    }
}

#[async_trait]
impl MediaDb for Tmdb {
    async fn query(
        &self,
        query: MediaQuery,
    ) -> Result<Paginated<Media>, crate::core::error::Error> {
        let tmdb_config = Arc::new(self.tmdb_api.config().await?);

        let query_plan: TmdbQueryPlan = query.clone().into();

        for item in query_plan.items.iter() {
            info!(self.logger, "\t{:?}", item);
        }

        let result = query_plan
            .execute(
                self.cache_db.clone(),
                self.tmdb_api.clone(),
                tmdb_config.clone(),
                &query,
            )
            .await?;

        Ok(result)
    }
}
