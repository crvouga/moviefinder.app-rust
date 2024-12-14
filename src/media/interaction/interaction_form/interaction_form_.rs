use serde::{Deserialize, Serialize};

use crate::{
    core::html::{button, children::text, div, Elem},
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

impl InteractionForm {
    pub fn view(self, media_id: &MediaId) -> Elem {
        div().class("w-full flex flex-row").children(
            to_available_interactions(self)
                .iter()
                .map(|(interaction_name, interaction_action)| {
                    interaction_name
                        .clone()
                        .view_bottom_button(media_id, interaction_action.clone())
                })
                .collect::<Vec<Elem>>(),
        )
    }
}

impl InteractionName {
    fn view_bottom_button(self, media_id: &MediaId, interaction_action: InteractionAction) -> Elem {
        let selected = match interaction_action {
            InteractionAction::Add => false,
            InteractionAction::Retract => true,
        };
        button()
            .class("flex flex-row h-16 flex-1 gap-1 items-center justify-center active:opacity-80 text-base font-bold")
            .data_class(|b| b.class("text-blue-500", &selected.to_string()))
            .child(self.view_icon(selected, "size-7"))
            .data_on(|e|e.click().sse(&Route::Record{
                interaction_action,
                interaction_name: self.clone(),
                media_id: media_id.clone(),
            }.url()))
            .child(text(&self.to_name()))
    }
}
