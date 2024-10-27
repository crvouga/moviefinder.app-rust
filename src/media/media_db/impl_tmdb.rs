use std::{sync::Arc, vec};

use super::interface::{MediaDb, MediaField};
use crate::{
    core::{
        pagination::{PageBased, Paginated, Pagination},
        query::{Filter, Op, Query},
    },
    media::{
        core::Media,
        media_id::MediaId,
        tmdb_api::{self, config::TmdbConfig, TmdbApi, TMDB_PAGE_SIZE},
    },
};
use async_trait::async_trait;
use futures::future::join_all;

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
    async fn query(&self, query: Query<MediaField>) -> Result<Paginated<Media>, String> {
        let tmdb_config = self.tmdb_api.config().await?;

        let query_plan = to_query_plan(query.clone(), vec![]);

        let result =
            execute_query_plan(&self.tmdb_api, &tmdb_config, query, query_plan.clone()).await?;

        println!("LOG {:?}", &query_plan);
        println!("LOG {:?}", result.items.len());

        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub enum QueryPlanItem {
    GetMovieDetails(MediaId),
    GetDiscoverMovie(Query<MediaField>),
}

impl QueryPlanItem {
    pub async fn execute(
        &self,
        tmdb_api: &TmdbApi,
        tmdb_config: &TmdbConfig,
    ) -> Result<Paginated<Media>, String> {
        match self {
            QueryPlanItem::GetMovieDetails(media_id) => {
                let movie_details_response = tmdb_api.movie_details(media_id.as_str()).await?;

                let movie = Media::from((tmdb_config, movie_details_response));

                Ok(Paginated {
                    items: vec![movie],
                    limit: 1,
                    offset: 0,
                    total: 1,
                })
            }
            QueryPlanItem::GetDiscoverMovie(query) => {
                let pagination: Pagination = query.into();

                let page_based: PageBased = (pagination, TMDB_PAGE_SIZE).into();

                let discover_requests = page_based
                    .range()
                    .map(|page| tmdb_api.discover_movie(page.into()));

                let discover_responses: Vec<tmdb_api::discover_movie::DiscoverMovieResponse> =
                    partition_results(join_all(discover_requests).await).unwrap_or_default();

                let items = discover_responses
                    .clone()
                    .into_iter()
                    .flat_map(|res| res.results.unwrap_or_default())
                    .skip(page_based.index + 1)
                    .take(query.limit)
                    .map(|result| Media::from((tmdb_config, result)))
                    .collect();

                let total = discover_responses
                    .into_iter()
                    .map(|res| res.total_results.unwrap_or(0))
                    .max()
                    .unwrap_or(0);

                Ok(Paginated {
                    items,
                    total,
                    limit: query.limit,
                    offset: query.limit,
                })
            }
        }
    }
}

fn partition_results<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>, Vec<E>> {
    let mut oks = Vec::new();
    let mut errs = Vec::new();

    for result in results {
        match result {
            Ok(val) => oks.push(val),
            Err(err) => errs.push(err),
        }
    }

    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(errs)
    }
}

pub type QueryPlan = Vec<QueryPlanItem>;

pub fn to_query_plan(query: Query<MediaField>, mut query_plan: QueryPlan) -> QueryPlan {
    match query.clone().filter {
        Filter::None => {
            let item = QueryPlanItem::GetDiscoverMovie(query);
            query_plan.push(item);
            query_plan
        }
        Filter::Clause(field, operator, value) => match (field, operator, value) {
            (MediaField::MediaId, Op::Eq, value) => {
                let media_id = MediaId::new(value);
                let item = QueryPlanItem::GetMovieDetails(media_id);
                query_plan.push(item);
                query_plan
            }
            _ => {
                query_plan.push(QueryPlanItem::GetDiscoverMovie(query.clone()));
                query_plan
            }
        },
        Filter::And(_filters) => query_plan,
        Filter::Or(_filters) => query_plan,
    }
}

pub async fn execute_query_plan(
    config: &TmdbApi,
    tmdb_config: &TmdbConfig,
    query: Query<MediaField>,
    query_plan: QueryPlan,
) -> Result<Paginated<Media>, String> {
    let mut all_items: Vec<Media> = vec![];
    let mut total = 0;

    for item in query_plan {
        let result = item.execute(config, tmdb_config).await?;
        total += result.total;
        all_items.extend(result.items);
    }

    Ok(Paginated {
        items: all_items,
        limit: query.limit,
        offset: query.offset,
        total,
    })
}
