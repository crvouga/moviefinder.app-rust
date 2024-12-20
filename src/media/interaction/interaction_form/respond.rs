use super::interaction_form_::{self, InteractionForm};
use super::route::Route;
use super::view_buttons::view_interaction_form_buttons;
use crate::core::html::Elem;
use crate::core::posix::Posix;
use crate::core::unit_of_work::uow;
use crate::info;
use crate::media::interaction::interaction_::MediaInteraction;
use crate::media::interaction::interaction_id::MediaInteractionId;
use crate::media::media_id::MediaId;
use crate::user::user_id::UserId;
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};
use std::collections::HashMap;

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
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
) -> Result<(), std::io::Error> {
    let interactions_by_media_id = get_interactions_by_media_id(ctx, user_id, media_ids).await;

    for (media_id, interactions) in interactions_by_media_id {
        let interaction_form = interaction_form_::derive(interactions);

        w.send_fragment(view_interaction_form(&media_id, Some(interaction_form)))
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

pub fn view_interaction_form(media_id: &MediaId, form: Option<InteractionForm>) -> Elem {
    view_interaction_form_buttons(media_id.clone(), form).id(&to_form_id(media_id))
}
