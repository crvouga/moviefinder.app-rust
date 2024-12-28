use crate::media::interaction::{
    interaction_::MediaInteraction,
    interaction_action::{InteractionAction, _to_all_interaction_actions},
    interaction_name::{to_all_interaction_names, InteractionName},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionForm {
    Initial,
    Seen,
    NotSeen,
    Interested,
    NotInterested,
    Liked,
    Disliked,
}

pub fn derive(mut interactions: Vec<MediaInteraction>) -> InteractionForm {
    interactions.sort_by_key(|k| k.created_at_posix);

    interactions
        .iter()
        .fold(InteractionForm::Initial, |_state, interaction| {
            to_next_interaction_form(
                interaction.interaction_name.clone(),
                interaction.interaction_action.clone(),
            )
        })
}

fn to_next_interaction_form(
    interaction_name: InteractionName,
    interaction_action: InteractionAction,
) -> InteractionForm {
    match (interaction_name, interaction_action) {
        (InteractionName::Seen, InteractionAction::Add) => InteractionForm::Seen,
        (InteractionName::Seen, InteractionAction::Retract) => InteractionForm::Initial,
        (InteractionName::NotSeen, InteractionAction::Add) => InteractionForm::NotSeen,
        (InteractionName::NotSeen, InteractionAction::Retract) => InteractionForm::Initial,
        (InteractionName::Liked, InteractionAction::Add) => InteractionForm::Liked,
        (InteractionName::Liked, InteractionAction::Retract) => InteractionForm::Seen,
        (InteractionName::Disliked, InteractionAction::Add) => InteractionForm::Disliked,
        (InteractionName::Disliked, InteractionAction::Retract) => InteractionForm::Seen,
        (InteractionName::Interested, InteractionAction::Add) => InteractionForm::Interested,
        (InteractionName::Interested, InteractionAction::Retract) => InteractionForm::NotSeen,
        (InteractionName::NotInterested, InteractionAction::Add) => InteractionForm::NotInterested,
        (InteractionName::NotInterested, InteractionAction::Retract) => InteractionForm::NotSeen,
    }
}

pub type Interaction = (InteractionName, InteractionAction);

pub fn _to_all_interactions() -> Vec<Interaction> {
    let mut all_interaction_buttons: Vec<Interaction> = vec![];

    for interaction_name in to_all_interaction_names() {
        for interaction_action in _to_all_interaction_actions() {
            all_interaction_buttons.push((interaction_name.clone(), interaction_action.clone()));
        }
    }

    all_interaction_buttons
}

pub fn to_available_interactions(interaction_form: InteractionForm) -> Vec<Interaction> {
    match interaction_form {
        InteractionForm::Initial => vec![
            (InteractionName::Seen, InteractionAction::Add),
            (InteractionName::NotSeen, InteractionAction::Add),
        ],
        InteractionForm::Seen => vec![
            (InteractionName::Seen, InteractionAction::Retract),
            (InteractionName::NotSeen, InteractionAction::Add),
            (InteractionName::Liked, InteractionAction::Add),
            (InteractionName::Disliked, InteractionAction::Add),
        ],
        InteractionForm::NotSeen => vec![
            (InteractionName::Seen, InteractionAction::Add),
            (InteractionName::NotSeen, InteractionAction::Retract),
            (InteractionName::Interested, InteractionAction::Add),
            (InteractionName::NotInterested, InteractionAction::Add),
        ],
        InteractionForm::Disliked => vec![
            (InteractionName::Seen, InteractionAction::Retract),
            (InteractionName::NotSeen, InteractionAction::Add),
            (InteractionName::Liked, InteractionAction::Add),
            (InteractionName::Disliked, InteractionAction::Retract),
        ],
        InteractionForm::Interested => vec![
            (InteractionName::Seen, InteractionAction::Add),
            (InteractionName::NotSeen, InteractionAction::Retract),
            (InteractionName::Interested, InteractionAction::Retract),
            (InteractionName::NotInterested, InteractionAction::Add),
        ],
        InteractionForm::Liked => vec![
            (InteractionName::Seen, InteractionAction::Retract),
            (InteractionName::NotSeen, InteractionAction::Add),
            (InteractionName::Liked, InteractionAction::Retract),
            (InteractionName::Disliked, InteractionAction::Add),
        ],
        InteractionForm::NotInterested => vec![
            (InteractionName::Seen, InteractionAction::Add),
            (InteractionName::NotSeen, InteractionAction::Retract),
            (InteractionName::Interested, InteractionAction::Add),
            (InteractionName::NotInterested, InteractionAction::Retract),
        ],
    }
}
