use super::route::Route;
use crate::core::posix::Posix;
use crate::core::unit_of_work::uow;
use crate::info;
use crate::media::interaction::interaction_::MediaInteraction;
use crate::media::interaction::interaction_action::InteractionAction;
use crate::media::interaction::interaction_id::MediaInteractionId;
use crate::media::media_id::MediaId;
use crate::ui::route::Routable;
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};
use crate::{
    core::{
        html::{button, children::text, div, span, style, unsafe_text, Elem},
        ui::icon_button::IconButton,
    },
    media::interaction::interaction_name::InteractionName,
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Form { media_id } => {
            w.send_fragment(view_form_bottom_bar(media_id)).await?;

            let user_id = r.user_id_result(ctx).await?;

            let _interactions = ctx
                .media_interaction_db
                .list_for_media(&user_id, media_id)
                .await?;

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
                None => return w.send_must_login_first().await,
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

            Ok(())
        }
    }
}

pub fn view_form_bottom_bar(media_id: &MediaId) -> Elem {
    div()
        .id(&format!("media-interaction-form-{}", media_id.as_str()))
        .class("flex flex-row items-center")
        .children(vec![
            InteractionName::Seen.view_bottom_button(media_id, false),
            InteractionName::NotSeen.view_bottom_button(media_id, false),
            // InteractionName::Liked.view_bottom_button(false),
            // InteractionName::Disliked.view_bottom_button(false),
            // InteractionName::Interested.view_bottom_button(false),
            // InteractionName::NotInterested.view_bottom_button(false),
        ])
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

pub fn _view_form_icon_buttons_vertical() -> Elem {
    div()
        .class("flex flex-col pb-2")
        .children(vec![
            InteractionName::Liked._view_icon_button(),
            InteractionName::Disliked._view_icon_button(),
            InteractionName::Interested._view_icon_button(),
            InteractionName::NotInterested._view_icon_button(),
            InteractionName::Seen._view_icon_button(),
            InteractionName::NotSeen._view_icon_button(),
        ])
        .child(style().child(unsafe_text(
            r#"
        .shadow { 
            filter: drop-shadow(0px 0px 6px black) drop-shadow(0px 0px 4px black); 
            text-shadow: 0px 0px 6px black, 0px 0px 4px black; 
        }
        "#,
        )))
}

impl InteractionName {
    fn _view_icon_button(self) -> Elem {
        let cloned = self.clone();
        IconButton::default()
            .label(self.to_name())
            .icon(move |_c| cloned.view_icon(true, "size-10 shadow"))
            .view()
            .class("flex flex-col gap-0.5 p-1.5")
            .child(
                span()
                    .class("text-xs font-bold shadow")
                    .child(text(&self.to_name())),
            )
    }
}
