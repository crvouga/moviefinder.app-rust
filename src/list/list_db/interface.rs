use crate::{
    core::{pagination::Paginated, query::Query, unit_of_work::UnitOfWork},
    list::core::list::List,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ListQueryField {
    #[default]
    UserId,
}

pub type ListQuery = Query<ListQueryField>;

#[async_trait]
pub trait ListDb: Send + Sync {
    async fn query(&self, query: ListQuery) -> Result<Paginated<List>, std::io::Error>;
    async fn upsert(&self, uow: UnitOfWork, list: List) -> Result<(), std::io::Error>;
}
