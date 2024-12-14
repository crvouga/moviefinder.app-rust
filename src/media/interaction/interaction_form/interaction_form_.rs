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
            println!("interaction: {:?}", interaction);
            println!(
                "interaction.interaction_name: {:?}",
                interaction.interaction_name
            );
            println!("");
            match interaction.interaction_name {
                InteractionName::Seen => InteractionForm::Seen,
                InteractionName::NotSeen => InteractionForm::NotSeen,
                InteractionName::Interested => InteractionForm::Interested,
                InteractionName::NotInterested => InteractionForm::NotInterested,
                InteractionName::Liked => InteractionForm::Liked,
                InteractionName::Disliked => InteractionForm::Disliked,
            }
        })
}

impl InteractionForm {
    pub fn view(self, media_id: &MediaId) -> Elem {
        match self {
            InteractionForm::Initial => view_root()
                .child(InteractionName::Seen.view_bottom_button(media_id, false))
                .child(InteractionName::NotSeen.view_bottom_button(media_id, false)),
            InteractionForm::Seen => view_root()
                .child(InteractionName::Seen.view_bottom_button(media_id, true))
                .child(InteractionName::Liked.view_bottom_button(media_id, false))
                .child(InteractionName::Disliked.view_bottom_button(media_id, false)),
            InteractionForm::NotSeen => view_root()
                .child(InteractionName::NotSeen.view_bottom_button(media_id, true))
                .child(InteractionName::Liked.view_bottom_button(media_id, false))
                .child(InteractionName::Disliked.view_bottom_button(media_id, false)),
            InteractionForm::Disliked => view_root()
                .child(InteractionName::Liked.view_bottom_button(media_id, false))
                .child(InteractionName::Disliked.view_bottom_button(media_id, true)),
            InteractionForm::Interested => view_root()
                .child(InteractionName::Seen.view_bottom_button(media_id, false))
                .child(InteractionName::NotSeen.view_bottom_button(media_id, false)),
            InteractionForm::Liked => view_root()
                .child(InteractionName::Liked.view_bottom_button(media_id, true))
                .child(InteractionName::Disliked.view_bottom_button(media_id, false)),
            InteractionForm::NotInterested => view_root()
                .child(InteractionName::Seen.view_bottom_button(media_id, false))
                .child(InteractionName::NotSeen.view_bottom_button(media_id, false)),
        }
    }
}

fn view_root() -> Elem {
    div().class("flex flex-row items-center w-full")
}

impl InteractionName {
    fn view_bottom_button(self, media_id: &MediaId, selected: bool) -> Elem {
        button()
            .class("flex flex-row h-16 flex-1 gap-1 items-center justify-center active:opacity-80 text-base font-bold")
            .data_class(|b| b.class("text-blue-500", if selected { "true" } else { "false" }))
            .child(self.view_icon(selected, "size-7"))
            .data_on(|e|e.click().sse(&Route::Record{
                interaction_action: InteractionAction::Add,
                interaction_name: self.clone(),
                media_id: media_id.clone(),
            }.url()))
            .child(text(&self.to_name()))
    }
}
