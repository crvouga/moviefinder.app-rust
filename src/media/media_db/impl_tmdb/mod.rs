use super::interface::{MediaDb, MediaQuery};
use crate::{
    core::{
        logger::interface::{Logger, LoggerDyn},
        pagination::Paginated,
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
    logger: LoggerDyn,
}

impl Tmdb {
    pub fn new(logger: LoggerDyn, tmdb_api: Arc<TmdbApi>) -> Tmdb {
        let logger_new = logger.child("impl_tmdb");
        Tmdb {
            tmdb_api,
            logger: logger_new,
        }
    }
}

#[async_trait]
impl MediaDb for Tmdb {
    async fn query(&self, query: MediaQuery) -> Result<Paginated<Media>, String> {
        let tmdb_config = self.tmdb_api.config().await?;

        let query_plan: TmdbQueryPlan = query.clone().into();

        info!(self.logger, "query= {:?}", query);
        info!(self.logger, "query_plan=");
        for item in query_plan.items.iter() {
            info!(self.logger, "\t{:?}", item);
        }

        let result = query_plan
            .execute(&self.tmdb_api, &tmdb_config, &query)
            .await?;

        Ok(result)
    }
}
