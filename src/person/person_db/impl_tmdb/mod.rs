use crate::{
    core::{
        logger::interface::Logger,
        pagination::Paginated,
        query::{QueryFilter, QueryOp},
        tmdb_api::{self, person::PersonResult, TMDB_PAGE_SIZE},
    },
    person::person_::Person,
};
use async_trait::async_trait;
use std::sync::Arc;

use super::interface::{PersonDb, PersonQuery, PersonQueryField};
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

impl From<PersonResult> for Person {
    fn from(person_result: PersonResult) -> Person {
        Person {
            id: person_result.id.unwrap_or(0).to_string(),
            name: person_result.name.unwrap_or("".to_string()),
        }
    }
}

impl ImplTmdb {
    async fn person_search(
        &self,
        query: &PersonQuery,
        search_query: &str,
    ) -> Result<Paginated<Person>, String> {
        let page_based = query.to_page_based(TMDB_PAGE_SIZE);

        let searched = self
            .tmdb_api
            .person_search(&search_query, &page_based.start_page)
            .await?;

        let items: Vec<Person> = searched
            .results
            .unwrap_or(vec![])
            .into_iter()
            .map(|p| p.into())
            .skip(query.offset)
            .take(query.limit)
            .collect();

        Ok(Paginated {
            items: items,
            total: searched.total_results.unwrap_or(0),
            limit: query.limit,
            offset: query.offset,
        })
    }

    async fn person_popular(&self, query: &PersonQuery) -> Result<Paginated<Person>, String> {
        let page_based = query.to_page_based(TMDB_PAGE_SIZE);

        let searched = self.tmdb_api.person_popular(page_based.start_page).await?;

        let items: Vec<Person> = searched
            .results
            .unwrap_or(vec![])
            .into_iter()
            .map(|p| p.into())
            .skip(query.offset)
            .take(query.limit)
            .collect();

        Ok(Paginated {
            items: items,
            total: searched.total_results.unwrap_or(0),
            limit: query.limit,
            offset: query.offset,
        })
    }
}

#[async_trait]
impl PersonDb for ImplTmdb {
    async fn query(&self, query: PersonQuery) -> Result<Paginated<Person>, String> {
        match &query.filter {
            QueryFilter::Clause(PersonQueryField::Name, QueryOp::Like, search_query) => {
                self.person_search(&query, search_query).await
            }

            _ => self.person_popular(&query).await,
        }
    }
}
