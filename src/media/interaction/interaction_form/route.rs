use serde::{Deserialize, Serialize};

use crate::media::{
    interaction::{interaction_action::InteractionAction, interaction_name::InteractionName},
    media_id::MediaId,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Form {
        media_id: MediaId,
    },
    Record {
        media_id: MediaId,
        name: InteractionName,
        action: InteractionAction,
    },
}
