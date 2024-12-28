use super::interaction_form_::to_available_interactions;
use super::interaction_form_::{self, InteractionForm};
use super::route::Route;
use crate::core::html::Html;
use crate::core::posix::Posix;
use crate::core::unit_of_work::uow;
use crate::info;
use crate::media::interaction::interaction_::MediaInteraction;
use crate::media::interaction::interaction_id::MediaInteractionId;
use crate::media::media_id::MediaId;
use crate::user::user_id::UserId;
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};
use crate::{
    core::{html::div, ui::labelled_icon_button::LabelledIconButton},
    media::interaction::{
        interaction_action::InteractionAction,
        interaction_name::{to_max_display_string_length, InteractionName},
    },
    ui::route::AppRoute,
};
use std::collections::HashMap;

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    match route {
        Route::Form { media_id } => {
            let user_id = r.user_id(ctx).await?;

            respond_interaction_form(ctx, w, user_id, vec![media_id.clone()]).await?;

            Ok(())
        }
        Route::Record {
            action: interaction_action,
            name: interaction_name,
            media_id,
        } => {
            let maybe_user_id = r.user_id(ctx).await.ok();

            let user_id = match maybe_user_id {
                Some(user_id) => user_id,
                None => {
                    return w
                        .respond_login_drawer("You'll have to login before interacting with media")
                        .await
                }
            };

            let interaction_new = MediaInteraction {
                id: MediaInteractionId::default(),
                created_at_posix: Posix::now(),
                interaction_name: interaction_name.clone(),
                interaction_action: interaction_action.clone(),
                media_id: media_id.clone(),
                user_id,
            };

            info!(ctx.log, "interaction_new: {:?}", interaction_new);

            ctx.media_interaction_db
                .put(uow(), &interaction_new)
                .await?;

            let user_id = r.user_id(ctx).await?;

            respond_interaction_form(ctx, w, user_id, vec![media_id.clone()]).await?;

            Ok(())
        }
    }
}
pub async fn respond_interaction_form(
    ctx: &Ctx,
    w: &mut ResponseWriter,
    user_id: UserId,
    media_ids: Vec<MediaId>,
) -> Result<(), crate::core::error::Error> {
    let interactions_by_media_id = get_interactions_by_media_id(ctx, user_id, media_ids).await;

    for (media_id, interactions) in interactions_by_media_id {
        let interaction_form = interaction_form_::derive(interactions);

        w.send_fragment(view(&media_id, Some(interaction_form)))
            .await?;
    }

    Ok(())
}

async fn get_interactions_by_media_id(
    ctx: &Ctx,
    user_id: UserId,
    media_ids: Vec<MediaId>,
) -> HashMap<MediaId, Vec<MediaInteraction>> {
    let mut interactions_by_media_id = media_ids
        .iter()
        .map(|id| (id.clone(), vec![]))
        .collect::<HashMap<MediaId, Vec<MediaInteraction>>>();

    let all_interactions = ctx
        .media_interaction_db
        .find_by_user_id_and_media_ids(&user_id, &media_ids.iter().collect())
        .await
        .unwrap_or_default();

    for i in all_interactions {
        interactions_by_media_id
            .entry(i.media_id.clone())
            .or_default()
            .push(i);
    }

    interactions_by_media_id
}

fn to_form_id(media_id: &MediaId) -> String {
    format!("media-interaction-form-{}", media_id.as_str())
}

pub fn view(media_id: &MediaId, form: Option<InteractionForm>) -> Html {
    div()
        .id(&to_form_id(media_id))
        .class("flex flex-col gap-2 pb-4")
        .child(view_width())
        .map(|e| match form {
            None => e.children(
                view_buttons_disabled()
                    .into_iter()
                    .take(4)
                    .collect::<Vec<Html>>(),
            ),

            Some(interaction_form) => {
                let available_interactions = to_available_interactions(interaction_form);

                e.children(
                    available_interactions
                        .iter()
                        .map(|(name, action)| view_icon_button_enabled(&action, &name, &media_id))
                        .chain(
                            view_buttons_disabled()
                                .into_iter()
                                .skip(available_interactions.len()),
                        )
                        .take(4)
                        .collect::<Vec<Html>>(),
                )
            }
        })
}

fn view_width() -> Html {
    div()
        .class("select-none opacity-0")
        .aria_hidden_true()
        .child_text(&"_".repeat(to_max_display_string_length()))
}

fn is_selected(action: &InteractionAction) -> bool {
    match action {
        InteractionAction::Add => false,
        InteractionAction::Retract => true,
    }
}

fn to_id(name: &InteractionName, action: &InteractionAction, disabled: bool) -> String {
    format!("{:?}-{:?}-{}", name, action, disabled)
}

fn view_icon_button_base(action: &InteractionAction, name: &InteractionName) -> LabelledIconButton {
    LabelledIconButton::default()
        .icon(name.view_icon(is_selected(action), "size-9"))
        .text(&name.to_display_string())
        .id(&to_id(name, action, false))
}

fn view_icon_button_enabled(
    action: &InteractionAction,
    name: &InteractionName,
    media_id: &MediaId,
) -> Html {
    view_icon_button_base(&action, &name)
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

fn view_icon_button_disabled(action: &InteractionAction, name: &InteractionName) -> Html {
    view_icon_button_base(action, name)
        .disabled(true)
        .view()
        .id(&to_id(name, action, true))
}

fn view_buttons_disabled() -> Vec<Html> {
    vec![
        (&InteractionAction::Add, &InteractionName::Seen),
        (&InteractionAction::Add, &InteractionName::NotSeen),
        (&InteractionAction::Add, &InteractionName::Liked),
        (&InteractionAction::Add, &InteractionName::Disliked),
        (&InteractionAction::Add, &InteractionName::Interested),
        (&InteractionAction::Add, &InteractionName::NotInterested),
    ]
    .into_iter()
    .map(|(action, name)| view_icon_button_disabled(action, name))
    .collect()
}
