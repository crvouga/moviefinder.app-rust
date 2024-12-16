use super::route::Route;
use crate::{
    core::{
        html::Elem,
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

pub fn view_bottom_buttons(media_id: MediaId, interaction_form: Option<InteractionForm>) -> Elem {
    BottomButtons::default()
        .view()
        .map(|e| match interaction_form {
            None => e.children(
                view_disabled_buttons()
                    .into_iter()
                    .take(4)
                    .collect::<Vec<Elem>>(),
            ),

            Some(interaction_form) => {
                let available_interactions = to_available_interactions(interaction_form);
                e.children(
                    available_interactions
                        .iter()
                        .map(|(interaction_name, interaction_action)| {
                            view_interaction_button(
                                interaction_action.clone(),
                                interaction_name.clone(),
                                media_id.clone(),
                            )
                        })
                        .chain(
                            view_disabled_buttons()
                                .into_iter()
                                .skip(available_interactions.len()),
                        )
                        .take(4)
                        .collect::<Vec<Elem>>(),
                )
            }
        })
}

fn is_selected(interaction_action: &InteractionAction) -> bool {
    match interaction_action {
        InteractionAction::Add => false,
        InteractionAction::Retract => true,
    }
}

fn to_id(
    interaction_name: &InteractionName,
    interaction_action: &InteractionAction,
    disabled: bool,
) -> String {
    format!(
        "{}-{}-{}",
        interaction_name.to_name(),
        interaction_action.to_string(),
        disabled
    )
}

fn view_interation_bottom_button(
    interaction_action: &InteractionAction,
    interaction_name: &InteractionName,
) -> BottomButton {
    BottomButton::default()
        .icon(interaction_name.view_icon(is_selected(interaction_action), "size-7"))
        .text(&interaction_name.to_name())
        .id(&to_id(interaction_name, interaction_action, false))
}

fn view_interaction_button(
    interaction_action: InteractionAction,
    interaction_name: InteractionName,
    media_id: MediaId,
) -> Elem {
    view_interation_bottom_button(&interaction_action, &interaction_name)
        .active(is_selected(&interaction_action))
        .view()
        .data_on(|e| {
            e.click().sse(
                &Route::Record {
                    interaction_action,
                    interaction_name,
                    media_id,
                }
                .url(),
            )
        })
}

fn view_disabled_buttons() -> Vec<Elem> {
    vec![
        (&InteractionAction::Add, &InteractionName::Seen),
        (&InteractionAction::Add, &InteractionName::NotSeen),
        (&InteractionAction::Add, &InteractionName::Liked),
        (&InteractionAction::Add, &InteractionName::Disliked),
        (&InteractionAction::Add, &InteractionName::Interested),
        (&InteractionAction::Add, &InteractionName::NotInterested),
    ]
    .into_iter()
    .map(|(action, name)| {
        view_interation_bottom_button(action, name)
            .disabled(true)
            .view()
            .id(&to_id(name, action, true))
    })
    .collect()
}
