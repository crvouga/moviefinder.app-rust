use super::{avatar, route::Route};
use crate::{
    core::{
        dynamic_data::DynamicData,
        html::{div, fieldset, form, Html},
        http::response_writer::ResponseWriter,
        js::Js,
        ui::{button::Button, text_field::TextField, top_bar::TopBar},
        unit_of_work::uow,
    },
    ctx::Ctx,
    req::Req,
    ui::{
        bottom_bar_form_buttons::{BottomBarFormButtons, SIGNAL_IS_SUBMITTING},
        route::AppRoute,
    },
    user::{
        self, account_screen, user_id::UserId, user_profile::user_profile_::UserProfile,
        username::Username,
    },
};
use std::time::Duration;

const SIGNAL_USERNAME: &str = "signal_username";
const SIGNAL_FULL_NAME: &str = "signal_full_name";

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    match route {
        Route::Screen { .. } => {
            let user_id = match r.user_id(ctx).await.ok().clone() {
                Some(user_id) => user_id,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            let maybe_profile = ctx.user_profile_db.find_one_by_user_id(&user_id).await?;

            let profile = match maybe_profile {
                Some(profile) => profile,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            w.send_screen(r, view_screen(profile.clone())).await?;

            w.send_signals(vec![
                (
                    SIGNAL_USERNAME,
                    &Js::str(&profile.username.clone().to_string()),
                ),
                (
                    SIGNAL_FULL_NAME,
                    &Js::str(&profile.full_name.clone().unwrap_or_default()),
                ),
            ])
            .await?;

            avatar::respond::respond_initial(ctx, r, w).await?;

            Ok(())
        }

        Route::InputtedUsername { .. } => {
            unimplemented!()
        }

        Route::SubmittedForm { .. } => {
            let username = r.payload.get_first(SIGNAL_USERNAME).unwrap_or_default();
            let full_name = r.payload.get_first(SIGNAL_FULL_NAME).unwrap_or_default();

            let avatar_seed = r
                .payload
                .get_first(avatar::respond::SIGNAL_AVATAR_SEED)
                .unwrap_or_default();

            let user_id = match r.user_id(ctx).await.ok() {
                Some(user_id) => user_id,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            let maybe_profile_existing = ctx.user_profile_db.find_one_by_user_id(&user_id).await?;

            let profile_existing = match maybe_profile_existing {
                Some(profile) => profile,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            let profile_new = UserProfile {
                avatar_seed: Some(avatar_seed.clone()),
                username: Username::from_string(username.clone()),
                full_name: Some(full_name.clone().trim().to_string()),
                ..profile_existing
            };

            ctx.user_profile_db.put(uow(), &profile_new).await?;

            user::account_screen::respond::redirect_to(ctx, r, w, &r.user_id(ctx).await.ok())
                .await?;

            w.send_toast_dark("Profile updated").await?;

            w.send_signal(SIGNAL_IS_SUBMITTING, "false").await?;

            Ok(())
        }

        Route::Avatar(c) => avatar::respond::respond(ctx, r, c, w).await,
    }
}

async fn respond_failed_to_load(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    w.send_toast_dark("User not found. Try logging in again")
        .await?;

    user::account_screen::respond::redirect_to(ctx, r, w, &None).await?;

    Ok(())
}

fn view_screen_root() -> Html {
    form()
        .class("flex flex-col w-full flex-1 overflow-hidden")
        .data_signal(SIGNAL_USERNAME, "''")
        .data_signal(SIGNAL_FULL_NAME, "''")
        .child_signals_json(false)
        .child(
            TopBar::default()
                .back_url(account_screen::route::Route::Screen.url())
                .title("Edit Profile")
                .view(),
        )
}

pub fn view_open_edit_profile_screen_button(user_id: UserId) -> Button {
    Button::default()
        .color_primary()
        .label("Edit Profile")
        .indicator("signal_edit_profile_button_indicator")
        .map_button(move |e| {
            e.data_on(|b| {
                b.press_up().push_url(
                    &Route::Screen {
                        user_id: user_id.clone(),
                    }
                    .url(),
                )
            })
        })
}

fn view_screen(profile: UserProfile) -> Html {
    view_screen_root()
        .map(|e| {
            let route = Route::SubmittedForm {
                user_id: profile.user_id.clone(),
            };

            let url = route.url();

            e.data_on(|b| b.submit().prevent_default().sse(&url))
        })
        .data_indicator(SIGNAL_IS_SUBMITTING)
        .child(
            div()
                .class("flex-1 w-full flex flex-col gap-12 p-6 overflow-y-scroll pb-48")
                .child(avatar::respond::view_fieldset(&profile))
                .child(
                    fieldset().child(
                        TextField::default()
                            .label("Username")
                            .map_input(move |i| {
                                i.data_bind(SIGNAL_USERNAME).data_on(|e| {
                                    e.change().debounce(Duration::from_secs(1 / 3)).sse(
                                        &Route::InputtedUsername {
                                            user_id: profile.user_id.clone(),
                                        }
                                        .url(),
                                    )
                                })
                            })
                            .placeholder("Username")
                            .view(),
                    ),
                )
                .child(
                    fieldset().child(
                        TextField::default()
                            .label("Full name")
                            .map_input(|i| i.data_bind(SIGNAL_FULL_NAME))
                            .placeholder("Full name")
                            .view(),
                    ),
                ),
        )
        .child(
            BottomBarFormButtons::default()
                .on_cancel(|e| {
                    e.press_up()
                        .push_url(&account_screen::route::Route::Screen.url())
                })
                .view()
                .id("bottom-bar-form"),
        )
}
