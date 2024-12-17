use super::{
    interaction_form_::{to_available_interactions, InteractionForm},
    route::Route,
};
use crate::{
    core::{
        html::Elem,
        ui::button_group::{ButtonGroup, ButtonGroupMember},
    },
    media::{
        interaction::{interaction_action::InteractionAction, interaction_name::InteractionName},
        media_id::MediaId,
    },
    ui::route::AppRoute,
};

pub fn view_interaction_form_buttons(
    media_id: MediaId,
    interaction_form: Option<InteractionForm>,
) -> Elem {
    ButtonGroup::default()
        .orientation_vertical()
        .view()
        .map(|e| match interaction_form {
            None => e.children(
                view_disabled_interaction_buttons()
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
                            view_disabled_interaction_buttons()
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
) -> ButtonGroupMember {
    ButtonGroupMember::default()
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
            e.press_down().sse(
                &Route::Record {
                    interaction_action,
                    interaction_name,
                    media_id,
                }
                .url(),
            )
        })
}

fn view_disabled_interaction_buttons() -> Vec<Elem> {
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
