use crate::core::query::Query;
use crate::{core::pagination::Paginated, media::person::person_::Person};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum PersonQueryField {
    #[default]
    Name,
}

pub type PersonQuery = Query<PersonQueryField>;

#[async_trait]
pub trait MediaPersonDb: Send + Sync {
    async fn query(
        &self,
        query: PersonQuery,
    ) -> Result<Paginated<Person>, crate::core::error::CoreError>;
}
