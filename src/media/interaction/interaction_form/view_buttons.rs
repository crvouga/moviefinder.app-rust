use super::{
    interaction_form_::{to_available_interactions, InteractionForm},
    route::Route,
};
use crate::{
    core::{
        html::{div, Elem},
        ui::labelled_icon_button::LabelledIconButton,
    },
    media::{
        interaction::{
            interaction_action::InteractionAction,
            interaction_name::{to_max_display_string_length, InteractionName},
        },
        media_id::MediaId,
    },
    ui::route::AppRoute,
};

pub fn view_interaction_form_buttons(
    media_id: MediaId,
    interaction_form: Option<InteractionForm>,
) -> Elem {
    div()
        .class("flex flex-col gap-2 pb-3")
        .child(
            div()
                .class("select-none opacity-0")
                .aria_hidden_true()
                .child_text(&"a".repeat(to_max_display_string_length())),
        )
        .map(|e| match interaction_form {
            None => e.children(
                view_interaction_buttons_disabled()
                    .into_iter()
                    .take(4)
                    .collect::<Vec<Elem>>(),
            ),

            Some(interaction_form) => {
                let available_interactions = to_available_interactions(interaction_form);

                e.children(
                    available_interactions
                        .iter()
                        .map(|(name, action)| {
                            view_interaction_button_enabled(&action, &name, &media_id)
                        })
                        .chain(
                            view_interaction_buttons_disabled()
                                .into_iter()
                                .skip(available_interactions.len()),
                        )
                        .take(4)
                        .collect::<Vec<Elem>>(),
                )
            }
        })
}

fn is_selected(action: &InteractionAction) -> bool {
    match action {
        InteractionAction::Add => false,
        InteractionAction::Retract => true,
    }
}

fn to_id(name: &InteractionName, action: &InteractionAction, disabled: bool) -> String {
    format!(
        "{}-{}-{}",
        name.to_display_string(),
        action.to_string(),
        disabled
    )
}

fn view_interaction_button(
    action: &InteractionAction,
    name: &InteractionName,
) -> LabelledIconButton {
    LabelledIconButton::default()
        .icon(name.view_icon(is_selected(action), "size-7"))
        .text(&name.to_display_string())
        .id(&to_id(name, action, false))
}

fn view_interaction_button_enabled(
    action: &InteractionAction,
    name: &InteractionName,
    media_id: &MediaId,
) -> Elem {
    view_interaction_button(&action, &name)
        .active(is_selected(&action))
        .view()
        .data_on(|e| {
            e.press_down().sse(
                &Route::Record {
                    action: action.clone(),
                    name: name.clone(),
                    media_id: media_id.clone(),
                }
                .url(),
            )
        })
}

fn view_interaction_button_disabled(action: &InteractionAction, name: &InteractionName) -> Elem {
    view_interaction_button(action, name)
        .disabled(true)
        .view()
        .id(&to_id(name, action, true))
}

fn view_interaction_buttons_disabled() -> Vec<Elem> {
    vec![
        (&InteractionAction::Add, &InteractionName::Seen),
        (&InteractionAction::Add, &InteractionName::NotSeen),
        (&InteractionAction::Add, &InteractionName::Liked),
        (&InteractionAction::Add, &InteractionName::Disliked),
        (&InteractionAction::Add, &InteractionName::Interested),
        (&InteractionAction::Add, &InteractionName::NotInterested),
    ]
    .into_iter()
    .map(|(action, name)| view_interaction_button_disabled(action, name))
    .collect()
}
