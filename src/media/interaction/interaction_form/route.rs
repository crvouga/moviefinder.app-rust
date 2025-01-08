use super::interaction_form_view_config::InteractionFormViewConfig;
use crate::media::interaction::{
    interaction_action::InteractionAction, interaction_name::InteractionName,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Form {
        view_config: InteractionFormViewConfig,
    },
    Record {
        name: InteractionName,
        action: InteractionAction,
        view_config: InteractionFormViewConfig,
    },
}
