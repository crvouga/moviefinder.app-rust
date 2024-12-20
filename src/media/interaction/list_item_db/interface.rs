use crate::{
    core::pagination::Paginated, list::list_item::ListItem,
    media::interaction::interaction_name::InteractionName, user::user_id::UserId,
};
use async_trait::async_trait;

#[async_trait]
pub trait ListItemDb: Send + Sync {
    async fn find_by_user_id_and_interaction_name(
        &self,
        limit: usize,
        offset: usize,
        user_id: UserId,
        interaction_name: InteractionName,
    ) -> Result<Paginated<ListItem>, std::io::Error>;
}
