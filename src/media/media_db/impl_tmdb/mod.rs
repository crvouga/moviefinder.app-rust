use super::interface::{MediaDb, MediaQuery};
use crate::{
    core::{logger::interface::Logger, pagination::Paginated},
    log_info,
    media::{core::Media, tmdb_api},
};
use async_trait::async_trait;
use std::sync::Arc;
use tmdb_query_plan::TmdbQueryPlan;

pub mod tmdb_query_plan;
pub mod tmdb_query_plan_item;
pub struct ImplTmdb {
    tmdb_api: Arc<tmdb_api::TmdbApi>,
    logger: Arc<dyn Logger>,
}

impl ImplTmdb {
    pub fn new(logger: Arc<dyn Logger>, tmdb_api: Arc<tmdb_api::TmdbApi>) -> ImplTmdb {
        let logger_new = logger.child("impl_tmdb");
        ImplTmdb {
            tmdb_api,
            logger: logger_new,
        }
    }
}

#[async_trait]
impl MediaDb for ImplTmdb {
    async fn query(&self, query: MediaQuery) -> Result<Paginated<Media>, String> {
        let tmdb_config = self.tmdb_api.config().await?;

        let query_plan: TmdbQueryPlan = query.clone().into();

        let result = query_plan
            .clone()
            .execute(&self.tmdb_api, &tmdb_config, &query)
            .await?;

        log_info!(
            self.logger,
            "\n\tquery={:?}\n\tquery_plan={:?}",
            query,
            query_plan
        );

        Ok(result)
    }
}
