use serde::{Deserialize, Serialize};

use crate::media::{
    interaction::{interaction_action::InteractionAction, interaction_name::InteractionName},
    media_id::MediaId,
};

use super::respond::InteractionFormOrientation;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Form {
        media_id: MediaId,
        orientation: InteractionFormOrientation,
    },
    Record {
        media_id: MediaId,
        name: InteractionName,
        action: InteractionAction,
        orientation: InteractionFormOrientation,
    },
}
