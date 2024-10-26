use std::vec;

use super::interface::{Field, MediaDb};
use crate::{
    core::{
        pagination::{PageBased, Paginated, Pagination},
        query::{Filter, Op, Query},
    },
    media::{
        core::Media,
        media_id::MediaId,
        tmdb_api::{self, config::TmdbConfig, Config, TMDB_PAGE_SIZE},
    },
};
use async_trait::async_trait;
use futures::future::join_all;

pub struct ImplTmdb {
    config: tmdb_api::Config,
}

impl ImplTmdb {
    pub fn new(tmdb_api_read_access_token: String) -> ImplTmdb {
        ImplTmdb {
            config: tmdb_api::Config::new(tmdb_api_read_access_token),
        }
    }
}

#[async_trait]
impl MediaDb for ImplTmdb {
    async fn query(&self, query: Query<Field>) -> Result<Paginated<Media>, String> {
        let tmdb_config = tmdb_api::config::load(&self.config).await?;

        let query_plan = to_query_plan(query.clone(), vec![]);

        let result =
            execute_query_plan(&self.config, &tmdb_config, query, query_plan.clone()).await?;

        println!("LOG {:?}", &query_plan);
        println!("LOG {:?}", result.items.len());

        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub enum QueryPlanItem {
    MovieDetails(MediaId),
    DiscoverMovie(Query<Field>),
}

impl QueryPlanItem {
    pub async fn execute(
        &self,
        config: &Config,
        tmdb_config: &TmdbConfig,
    ) -> Result<Paginated<Media>, String> {
        match self {
            QueryPlanItem::MovieDetails(media_id) => {
                let movie_details_response =
                    tmdb_api::movie_details::send(config, media_id.as_str()).await?;

                let movie = Media::from((tmdb_config, movie_details_response));

                Ok(Paginated {
                    items: vec![movie],
                    limit: 1,
                    offset: 0,
                    total: 1,
                })
            }
            QueryPlanItem::DiscoverMovie(query) => {
                let pagination: Pagination = query.into();

                let page_based: PageBased = (pagination, TMDB_PAGE_SIZE).into();

                let discover_requests = page_based
                    .range()
                    .map(|page| tmdb_api::discover_movie::send(config, page.into()));

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

pub fn to_query_plan(query: Query<Field>, mut query_plan: QueryPlan) -> QueryPlan {
    match query.filter {
        Filter::None => {
            let item = QueryPlanItem::DiscoverMovie(query);
            query_plan.push(item);
            query_plan
        }
        Filter::Clause(clause) => match (clause.field, clause.operator, clause.value) {
            (Field::MediaId, Op::Eq, value) => {
                let media_id = MediaId::new(value);
                let item = QueryPlanItem::MovieDetails(media_id);
                query_plan.push(item);
                query_plan
            }
        },
    }
}

pub async fn execute_query_plan(
    config: &Config,
    tmdb_config: &TmdbConfig,
    query: Query<Field>,
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
