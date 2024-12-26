use crate::{
    core::pagination::{Paginated, Pagination},
    list::{list::List, list_item::ListItem},
};
use async_trait::async_trait;

#[async_trait]
pub trait ListScreenDb<TList: List + Clone + 'static>: Send + Sync {
    async fn find_list_items(
        &self,
        pagination: Pagination,
        list: TList,
    ) -> Result<Paginated<ListItem>, crate::core::error::Error>;
}
