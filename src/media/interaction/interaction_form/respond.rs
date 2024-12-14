use super::interaction_form_::{self, InteractionForm};
use super::route::Route;
use crate::core::html::{div, Elem};
use crate::core::posix::Posix;
use crate::core::unit_of_work::uow;
use crate::info;
use crate::media::interaction::interaction_::MediaInteraction;
use crate::media::interaction::interaction_id::MediaInteractionId;
use crate::media::media_id::MediaId;
use crate::ui::route::Routable;
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Form { media_id } => {
            respond_interaction_form(ctx, r, route, w, media_id).await?;

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

            respond_interaction_form(ctx, r, route, w, media_id).await?;

            Ok(())
        }
    }
}
pub async fn respond_interaction_form(
    ctx: &Ctx,
    r: &Req,
    _route: &Route,
    w: &mut ResponseWriter,
    media_id: &MediaId,
) -> Result<(), std::io::Error> {
    let user_id = r.user_id_result(ctx).await?;

    let interactions = ctx
        .media_interaction_db
        .list_by_user_media(&user_id, media_id)
        .await?;

    let interaction_form = interaction_form_::derive(interactions);

    w.send_fragment(view_interaction_form(media_id, interaction_form))
        .await?;

    Ok(())
}

pub fn view_root(media_id: &MediaId) -> Elem {
    div()
        .id(&format!("media-interaction-form-{}", media_id.as_str()))
        .class("h-16 w-full flex flex-row items-center")
}

pub fn view_interaction_form_load(media_id: &MediaId) -> Elem {
    view_root(media_id).data_intersects(|e| {
        e.sse(
            &Route::Form {
                media_id: media_id.clone(),
            }
            .url(),
        )
    })
}

fn view_interaction_form(media_id: &MediaId, interaction_form: InteractionForm) -> Elem {
    view_root(media_id).child(interaction_form.view(media_id))
}
