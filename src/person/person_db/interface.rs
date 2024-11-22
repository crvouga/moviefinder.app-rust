use crate::core::query::Query;
use crate::{core::pagination::Paginated, person::person_::Person};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum PersonQueryField {
    #[default]
    Name,
}

pub type PersonQuery = Query<PersonQueryField>;

#[async_trait]
pub trait PersonDb: Send + Sync {
    async fn query(&self, query: PersonQuery) -> Result<Paginated<Person>, String>;
}
