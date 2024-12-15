use super::{form_state::FormState, route::Route};
use crate::{
    core::{
        html::{div, fieldset, Elem},
        http::response_writer::ResponseWriter,
        js::Js,
        random,
        ui::{
            avatar::Avatar, button::Button, icon, icon_button::IconButton, text_field::TextField,
        },
    },
    ctx::Ctx,
    req::Req,
    ui::route::Routable,
    user::user_profile::user_profile_::{js_avatar_url, UserProfile},
};

pub const SIGNAL_AVATAR_SEED: &str = "signal_avatar_seed";
const SIGNAL_AVATAR_SEED_CAN_UNDO: &str = "signal_avatar_seed_can_undo";
const SIGNAL_AVATAR_SEED_CAN_REDO: &str = "signal_avatar_seed_can_redo";

pub async fn respond_initial(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    let profile = r.profile(ctx).await.unwrap_or_default();

    let mut form_state = FormState::get(&ctx, &profile).await;

    form_state
        .history
        .clear(profile.avatar_seed.clone().unwrap_or_default());

    send_form_state(&form_state, w).await?;

    FormState::put(&ctx, &profile, &form_state).await?;

    Ok(())
}

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::ClickedRandomSeed => {
            let avatar_seed_new = random::string(32);

            w.send_signal(SIGNAL_AVATAR_SEED, &Js::quote(&avatar_seed_new))
                .await?;

            let profile = r.profile(ctx).await.unwrap_or_default();

            let mut form_state = FormState::get(&ctx, &profile).await;

            form_state.history.push(avatar_seed_new);

            send_form_state(&form_state, w).await?;

            FormState::put(&ctx, &profile, &form_state).await?;

            Ok(())
        }

        Route::ClickedRedoSeed => {
            let profile = r.profile(ctx).await.unwrap_or_default();

            let mut form_state = FormState::get(&ctx, &profile).await;

            form_state.history.redo();

            send_form_state(&form_state, w).await?;

            FormState::put(&ctx, &profile, &form_state).await?;

            Ok(())
        }

        Route::ClickedUndoSeed => {
            let profile = r.profile(ctx).await.unwrap_or_default();

            let mut form_state = FormState::get(&ctx, &profile).await;

            form_state.history.undo();

            send_form_state(&form_state, w).await?;

            FormState::put(&ctx, &profile, &form_state).await?;

            Ok(())
        }
    }
}

async fn send_form_state(
    form_state: &FormState,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    w.send_signal(
        SIGNAL_AVATAR_SEED,
        &Js::quote(&form_state.history.present()),
    )
    .await?;

    w.send_signal(
        SIGNAL_AVATAR_SEED_CAN_UNDO,
        &form_state.history.can_undo().to_string(),
    )
    .await?;

    w.send_signal(
        SIGNAL_AVATAR_SEED_CAN_REDO,
        &form_state.history.can_redo().to_string(),
    )
    .await?;

    Ok(())
}

pub fn view_fieldset(_profile: &UserProfile) -> Elem {
    fieldset()
        .data_signal(SIGNAL_AVATAR_SEED, "''")
        .data_signal(SIGNAL_AVATAR_SEED_CAN_REDO, "false")
        .data_signal(SIGNAL_AVATAR_SEED_CAN_UNDO, "false")
        .class("flex flex-col w-full gap-4 items-center justify-center")
        .child(
            Avatar::default()
                .data_attributes_src(&format!(
                    "{}.value.trim().length === 0 ? null : {}",
                    SIGNAL_AVATAR_SEED,
                    js_avatar_url(&Js::dot_value(SIGNAL_AVATAR_SEED))
                ))
                .class("size-36")
                .view(),
        )
        .child(
            TextField::default()
                .label("Avatar Seed")
                .map_input(|i| i.data_bind(SIGNAL_AVATAR_SEED))
                .placeholder("Avatar Seed")
                .view(),
        )
        .child(
            div()
                .class("w-full flex items-center justify-center")
                .child(
                    div()
                        .class("flex-1 flex items-center justify-start")
                        .child(
                            IconButton::default()
                                .label("Undo".to_owned())
                                .icon(|class: String| icon::solid::rotate_left(&class))
                                .bind_disabled(Js::not(&Js::dot_value(SIGNAL_AVATAR_SEED_CAN_UNDO)))
                                .view()
                                .data_on(|e| e.click().sse(&Route::ClickedUndoSeed.url())),
                        )
                        .child(
                            IconButton::default()
                                .label("Redo".to_owned())
                                .icon(|class| icon::solid::rotate_right(&class))
                                .bind_disabled(Js::not(&Js::dot_value(SIGNAL_AVATAR_SEED_CAN_REDO)))
                                .view()
                                .data_on(|e| e.click().sse(&Route::ClickedRedoSeed.url())),
                        ),
                )
                .child(
                    Button::default()
                        .label("Random seed")
                        .view()
                        .data_on(|b| b.click().sse(&Route::ClickedRandomSeed.url())),
                ),
        )
}
