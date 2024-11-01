use super::tmdb_query_plan_item::TmdbQueryPlanItem;
use crate::{
    core::{
        pagination::Paginated,
        query::{Filter, Op},
    },
    media::{
        core::Media,
        media_db::interface::{MediaField, MediaQuery},
        media_id::MediaId,
        tmdb_api::{config::TmdbConfig, TmdbApi},
    },
};
use std::vec;

#[derive(Debug, Clone, Default)]
pub struct TmdbQueryPlan {
    items: Vec<TmdbQueryPlanItem>,
}

impl TmdbQueryPlan {
    pub async fn execute(
        self,
        config: &TmdbApi,
        tmdb_config: &TmdbConfig,
        query: &MediaQuery,
    ) -> Result<Paginated<Media>, String> {
        let mut all_items: Vec<Media> = vec![];
        let mut total = 0;

        for item in self.items.iter() {
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
}

impl From<MediaQuery> for TmdbQueryPlan {
    fn from(media_query: MediaQuery) -> TmdbQueryPlan {
        let mut query_plan = TmdbQueryPlan::default();

        match media_query.filter.clone() {
            Filter::None => {
                let item = TmdbQueryPlanItem::GetDiscoverMovie(media_query.clone().into());
                query_plan.items.push(item);
                query_plan
            }
            Filter::Clause(field, operator, value) => match (field, operator, value) {
                (MediaField::MediaId, Op::Eq, value) => {
                    let media_id = MediaId::new(value);
                    let item = TmdbQueryPlanItem::GetMovieDetails { media_id };
                    query_plan.items.push(item);
                    query_plan
                }
                _ => {
                    let item = TmdbQueryPlanItem::GetDiscoverMovie(media_query.clone().into());
                    query_plan.items.push(item);
                    query_plan
                }
            },
            Filter::And(_filters) => {
                let item = TmdbQueryPlanItem::GetDiscoverMovie(media_query.clone().into());
                query_plan.items.push(item);
                query_plan
            }
            Filter::Or(_filters) => {
                let item = TmdbQueryPlanItem::GetDiscoverMovie(media_query.clone().into());
                query_plan.items.push(item);
                query_plan
            }
        }
    }
}
