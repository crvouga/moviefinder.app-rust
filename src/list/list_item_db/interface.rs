use crate::{
    core::{pagination::Paginated, query::Query, unit_of_work::UnitOfWork},
    list::core::list_item::ListItem,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ListItemQueryField {
    #[default]
    ParentListId,
}

pub type ListItemQuery = Query<ListItemQueryField>;

#[async_trait]
pub trait ListItemDb: Send + Sync {
    async fn query(&self, query: ListItemQuery) -> Result<Paginated<ListItem>, std::io::Error>;
    async fn upsert(
        &self,
        uow: UnitOfWork,
        list_items: Vec<ListItem>,
    ) -> Result<(), std::io::Error>;
}
