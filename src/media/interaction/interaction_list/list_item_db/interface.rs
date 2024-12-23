use crate::{
    core::pagination::{Paginated, Pagination},
    list::list_item::MediaListItem,
    media::interaction::interaction_name::InteractionName,
    user::user_id::UserId,
};
use async_trait::async_trait;

#[async_trait]
pub trait MediaInteractionListItemDb: Send + Sync {
    async fn find_by_user_id_and_interaction_name(
        &self,
        pagination: Pagination,
        user_id: UserId,
        interaction_name: InteractionName,
    ) -> Result<Paginated<MediaListItem>, crate::core::error::Error>;
}
