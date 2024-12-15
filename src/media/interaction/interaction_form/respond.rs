use super::interaction_form_::{self, InteractionForm};
use super::route::Route;
use crate::core::html::{div, Elem};
use crate::core::posix::Posix;
use crate::core::unit_of_work::uow;
use crate::info;
use crate::media::interaction::interaction_::MediaInteraction;
use crate::media::interaction::interaction_id::MediaInteractionId;
use crate::media::media_id::MediaId;
use crate::user::user_id::UserId;
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Form { media_id } => {
            let user_id = r.user_id_result(ctx).await?;

            respond_interaction_form(ctx, w, user_id, vec![media_id.clone()]).await?;

            Ok(())
        }
        Route::Record {
            interaction_action,
            interaction_name,
            media_id,
        } => {
            let maybe_user_id = r.user_id(ctx).await;

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

            info!(ctx.logger, "interaction_new: {:?}", interaction_new);

            ctx.media_interaction_db
                .put(uow(), &interaction_new)
                .await?;

            let user_id = r.user_id_result(ctx).await?;

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
    let interactions_by_media_od = ctx
        .media_interaction_db
        .list_by_user_media(&user_id, &media_ids.iter().collect())
        .await?
        .into_iter()
        .fold(std::collections::BTreeMap::new(), |mut acc, i| {
            acc.entry(i.media_id.clone()).or_insert(vec![]).push(i);
            acc
        });

    for (media_id, all_interactions) in interactions_by_media_od {
        let interactions = all_interactions
            .into_iter()
            .filter(|i| i.media_id == media_id)
            .collect();

        let interaction_form = interaction_form_::derive(interactions);

        w.send_fragment(view_interaction_form(&media_id, Some(interaction_form)))
            .await?;
    }

    Ok(())
}

pub fn view_interaction_form(media_id: &MediaId, f: Option<InteractionForm>) -> Elem {
    div()
        .id(&format!("media-interaction-form-{}", media_id.as_str()))
        .class("h-fit w-full")
        .child(interaction_form_::view_bottom_buttons(media_id.clone(), f))
}
