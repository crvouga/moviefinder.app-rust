use crate::{
    core::pagination::Paginated,
    list::{list::List, list_item::ListItem},
};
use async_trait::async_trait;

#[async_trait]
pub trait ListScreenDb<TList: List + Clone + 'static> {
    async fn find_list_items(
        &self,
        offset: usize,
        limit: usize,
        list: TList,
    ) -> Result<Paginated<ListItem>, std::io::Error>;
}
