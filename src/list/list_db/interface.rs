use crate::{
    core::{pagination::Paginated, query::Query},
    list::list_joined::ListJoined,
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
    async fn query(&self, query: ListQuery) -> Result<Paginated<ListJoined>, String>;
}
