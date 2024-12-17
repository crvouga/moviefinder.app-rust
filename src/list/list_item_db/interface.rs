use crate::{
    core::{pagination::Paginated, query::Query, unit_of_work::UnitOfWork},
    list::core::{list_id::ListId, list_item::ListItem},
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
    async fn find_by_list_id(
        &self,
        limit: usize,
        offset: usize,
        list_id: ListId,
    ) -> Result<Paginated<ListItem>, std::io::Error>;
    async fn put(&self, uow: UnitOfWork, list_items: Vec<ListItem>) -> Result<(), std::io::Error>;
}
