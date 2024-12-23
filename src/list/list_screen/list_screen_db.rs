use crate::{
    core::pagination::{Paginated, Pagination},
    list::{list::MediaList, list_item::MediaListItem},
};
use async_trait::async_trait;

#[async_trait]
pub trait MediaListScreenDb<TList: MediaList + Clone + 'static>: Send + Sync {
    async fn find_list_items(
        &self,
        pagination: Pagination,
        list: TList,
    ) -> Result<Paginated<MediaListItem>, crate::core::error::Error>;
}
