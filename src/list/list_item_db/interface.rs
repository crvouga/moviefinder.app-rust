use crate::{
    core::pagination::Paginated,
    list::{list_id::ListId, list_item::ListItem},
};
use async_trait::async_trait;

#[async_trait]
pub trait ListItemDb: Send + Sync {
    async fn find_by_list_id(
        &self,
        limit: usize,
        offset: usize,
        list_id: ListId,
    ) -> Result<Paginated<ListItem>, std::io::Error>;
}
