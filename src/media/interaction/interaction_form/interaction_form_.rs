use serde::{Deserialize, Serialize};

use crate::{
    core::{
        html::{button, children::text, div, Elem},
        ui::bottom_bar_buttons::{BottomButton, BottomButtons},
    },
    media::{
        interaction::{
            interaction_::MediaInteraction, interaction_action::InteractionAction,
            interaction_name::InteractionName,
        },
        media_id::MediaId,
    },
    ui::route::Routable,
};

use super::route::Route;

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

pub fn to_available_interactions(
    interaction_form: InteractionForm,
) -> Vec<(InteractionName, InteractionAction)> {
    match interaction_form {
        InteractionForm::Initial => vec![
            (InteractionName::Seen, InteractionAction::Add),
            (InteractionName::NotSeen, InteractionAction::Add),
        ],
        InteractionForm::Seen => vec![
            (InteractionName::Seen, InteractionAction::Retract),
            (InteractionName::Liked, InteractionAction::Add),
            (InteractionName::Disliked, InteractionAction::Add),
        ],
        InteractionForm::NotSeen => vec![
            (InteractionName::NotSeen, InteractionAction::Retract),
            (InteractionName::Interested, InteractionAction::Add),
            (InteractionName::NotInterested, InteractionAction::Add),
        ],
        InteractionForm::Disliked => vec![
            (InteractionName::Seen, InteractionAction::Retract),
            (InteractionName::Liked, InteractionAction::Add),
            (InteractionName::Disliked, InteractionAction::Retract),
        ],
        InteractionForm::Interested => vec![
            (InteractionName::NotSeen, InteractionAction::Retract),
            (InteractionName::Interested, InteractionAction::Retract),
            (InteractionName::NotInterested, InteractionAction::Add),
        ],
        InteractionForm::Liked => vec![
            (InteractionName::Seen, InteractionAction::Retract),
            (InteractionName::Liked, InteractionAction::Retract),
            (InteractionName::Disliked, InteractionAction::Add),
        ],
        InteractionForm::NotInterested => vec![
            (InteractionName::NotSeen, InteractionAction::Retract),
            (InteractionName::Interested, InteractionAction::Add),
            (InteractionName::NotInterested, InteractionAction::Retract),
        ],
    }
}

pub fn view(media_id: MediaId, interaction_form: Option<InteractionForm>) -> Elem {
    BottomButtons::default()
        .view()
        .map(|e| match interaction_form {
            None => e,

            Some(interaction_form) => e.children(
                to_available_interactions(interaction_form)
                    .iter()
                    .map(|(interaction_name, interaction_action)| {
                        let selected = match interaction_action {
                            InteractionAction::Add => false,
                            InteractionAction::Retract => true,
                        };
                        BottomButton::default()
                            .active(selected)
                            .icon(interaction_name.view_icon(selected, "size-7"))
                            .text(&interaction_name.to_name())
                            .view()
                            .data_on(|e| {
                                e.click().sse(
                                    &Route::Record {
                                        interaction_action: interaction_action.clone(),
                                        interaction_name: interaction_name.clone(),
                                        media_id: media_id.clone(),
                                    }
                                    .url(),
                                )
                            })
                    })
                    .collect::<Vec<Elem>>(),
            ),
        })
}
