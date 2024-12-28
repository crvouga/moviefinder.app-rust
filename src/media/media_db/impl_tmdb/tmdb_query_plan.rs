use std::sync::Arc;

use super::tmdb_query_plan_item::TmdbQueryPlanItem;
use crate::{
    core::{
        cache_db::interface::CacheDbDyn,
        pagination::Paginated,
        query::{QueryFilter, QueryOp},
        tmdb_api::{config::TmdbConfig, TmdbApi},
    },
    media::{
        media_::Media,
        media_db::interface::{MediaQuery, MediaQueryField},
        media_id::MediaId,
    },
};

#[derive(Debug, Clone, Default)]
pub struct TmdbQueryPlan {
    pub items: Vec<TmdbQueryPlanItem>,
}

impl TmdbQueryPlan {
    pub async fn execute(
        self,
        cache_db: CacheDbDyn,
        tmdb_api: Arc<TmdbApi>,
        tmdb_config: Arc<TmdbConfig>,
        query: &MediaQuery,
    ) -> Result<Paginated<Media>, crate::core::error::Error> {
        let mut all_items: Vec<Media> = vec![];
        let mut total = 0;

        for item in self.items.iter() {
            let result = item
                .execute(cache_db.clone(), tmdb_api.clone(), tmdb_config.clone())
                .await?;
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
}

impl From<MediaQuery> for TmdbQueryPlan {
    fn from(media_query: MediaQuery) -> TmdbQueryPlan {
        to_tmdb_query_plan_step(media_query.clone(), media_query.filter.clone())
    }
}

fn to_tmdb_query_plan_step(
    media_query: MediaQuery,
    filter: QueryFilter<MediaQueryField>,
) -> TmdbQueryPlan {
    let mut query_plan = TmdbQueryPlan::default();

    match filter.clone() {
        QueryFilter::None => {
            let item = TmdbQueryPlanItem::GetDiscoverMovie {
                params: media_query.clone().into(),
            };
            query_plan.items.push(item);
            query_plan
        }
        QueryFilter::Clause(field, operator, value) => match (field, operator, value) {
            (MediaQueryField::MediaId, QueryOp::Eq, value) => {
                let media_id = MediaId::new(value);
                let item = TmdbQueryPlanItem::GetMovieDetails { media_id };
                query_plan.items.push(item);
                query_plan
            }

            _ => {
                let item = TmdbQueryPlanItem::GetDiscoverMovie {
                    params: media_query.clone().into(),
                };
                query_plan.items.push(item);
                query_plan
            }
        },
        QueryFilter::And(_filters) => {
            let item = TmdbQueryPlanItem::GetDiscoverMovie {
                params: media_query.clone().into(),
            };
            query_plan.items.push(item);
            query_plan
        }
        QueryFilter::Or(filters) => {
            for f in filters {
                let plan = to_tmdb_query_plan_step(media_query.clone(), f);
                query_plan.items.extend(plan.items);
            }
            query_plan
        }
    }
}
