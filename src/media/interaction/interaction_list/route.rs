use crate::{media::interaction::interaction_name::InteractionName, user::user_id::UserId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    ListsSection {
        user_id: UserId,
    },
    ListScreen {
        user_id: UserId,
        name: InteractionName,
    },
}
