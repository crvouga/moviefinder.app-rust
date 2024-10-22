use std::vec;

use super::{
    tmdb_api::{self, config::TmdbConfig, Config},
    Field, MediaDb,
};
use crate::{
    core::{
        pagination::Paginated,
        query::{Filter, Op, Query},
    },
    media::{media::Media, media_id::MediaId},
};
use async_trait::async_trait;

pub struct Tmdb {
    config: tmdb_api::Config,
}

impl Tmdb {
    pub fn new(tmdb_api_read_access_token: String) -> Tmdb {
        Tmdb {
            config: tmdb_api::Config::new(tmdb_api_read_access_token),
        }
    }
}

#[derive(Debug)]
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
                    tmdb_api::movie_details::send(&config, media_id.as_str()).await?;

                let movie = Media::from((tmdb_config, movie_details_response));

                Ok(Paginated {
                    items: vec![movie],
                    limit: 1,
                    offset: 0,
                    total: 1,
                })
            }
            QueryPlanItem::DiscoverMovie(query) => {
                let discover_response = tmdb_api::discover_movie::send(&config).await?;

                let items = discover_response
                    .results
                    .unwrap_or_default()
                    .into_iter()
                    .map(|result| Media::from((tmdb_config, result)))
                    .collect();

                Ok(Paginated {
                    items,
                    limit: query.limit,
                    offset: query.limit,
                    total: discover_response.total_results.unwrap_or(0),
                })
            }
        }
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

#[async_trait]
impl MediaDb for Tmdb {
    async fn query(&self, query: Query<Field>) -> Result<Paginated<Media>, String> {
        let tmdb_config = tmdb_api::config::load(&self.config).await?;

        let query_plan = to_query_plan(query.clone(), vec![]);

        let result = execute_query_plan(&self.config, &tmdb_config, query, query_plan).await?;

        Ok(result)
    }
}
