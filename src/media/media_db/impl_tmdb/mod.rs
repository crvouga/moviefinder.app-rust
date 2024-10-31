use super::interface::{MediaDb, MediaQuery};
use crate::{
    core::pagination::Paginated,
    media::{core::Media, tmdb_api},
};
use async_trait::async_trait;
use std::sync::Arc;
use tmdb_query_plan::TmdbQueryPlan;

pub mod tmdb_query_plan;
pub mod tmdb_query_plan_item;
pub struct ImplTmdb {
    tmdb_api: Arc<tmdb_api::TmdbApi>,
}

impl ImplTmdb {
    pub fn new(tmdb_api: Arc<tmdb_api::TmdbApi>) -> ImplTmdb {
        ImplTmdb { tmdb_api }
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

        println!("LOG query={:?}", &query);
        // println!("LOG query_plan={:?}", &query_plan);
        // println!("LOG result.items={:?}", result.items.len());

        Ok(result)
    }
}
