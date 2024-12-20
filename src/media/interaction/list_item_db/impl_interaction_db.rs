use super::interface::ListItemDb;
use crate::{
    core::pagination::Paginated,
    list::list_item::ListItem,
    media::interaction::{
        interaction_db::interface::MediaInteractionDb, interaction_name::InteractionName,
    },
    user::user_id::UserId,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ImplInteractionDb {
    interaction_db: Arc<dyn MediaInteractionDb>,
}

impl ImplInteractionDb {
    #[allow(dead_code)]
    pub fn new(interaction_db: Arc<dyn MediaInteractionDb>) -> Self {
        Self { interaction_db }
    }
}

#[async_trait]
impl ListItemDb for ImplInteractionDb {
    async fn find_by_user_id_and_interaction_name(
        &self,
        limit: usize,
        offset: usize,
        user_id: UserId,
        interaction_name: InteractionName,
    ) -> Result<Paginated<ListItem>, std::io::Error> {
        let _interactions = self
            .interaction_db
            .find_by_user_id_and_interaction_name(&user_id, &interaction_name)
            .await?;

        let empty_response = Ok(Paginated {
            items: vec![],
            total: 0,
            limit,
            offset,
        });

        empty_response
    }
}
