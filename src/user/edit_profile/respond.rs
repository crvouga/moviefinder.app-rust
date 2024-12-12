use std::time::Duration;

use super::{form_state::FormState, route::Route};
use crate::{
    core::{
        datastar::datastar::{js_dot_value, js_quote},
        dynamic_data::DynamicData,
        html::{div, fieldset, form, Elem},
        http::response_writer::ResponseWriter,
        random,
        ui::{
            avatar::Avatar, button::Button, icon, icon_button::IconButton, spinner_page,
            text_field::TextField, top_bar::TopBar,
        },
        unit_of_work::uow,
    },
    ctx::Ctx,
    req::Req,
    ui::{
        bottom_bar_form::{BottomBarForm, SIGNAL_IS_SUBMITTING},
        route::Routable,
    },
    user::{
        self,
        user_profile::user_profile_::{js_avatar_url, UserProfile},
        username::Username,
    },
};

const SIGNAL_USERNAME: &str = "signal_username";
const SIGNAL_FULL_NAME: &str = "signal_full_name";
const SIGNAL_AVATAR_SEED: &str = "signal_avatar_seed";

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen { .. } => {
            w.send_screen(view_screen_loading()).await?;

            let user_id = match r.user_id(ctx).await.clone() {
                Some(user_id) => user_id,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            let maybe_profile = ctx.user_profile_db.find_one_by_user_id(&user_id).await?;

            let profile = match maybe_profile {
                Some(profile) => profile,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            w.send_screen(view_screen(profile.clone())).await?;

            let mut form_state = FormState::get(ctx, &profile).await;
            form_state.reset(&profile);

            w.send_signals(vec![
                (
                    SIGNAL_USERNAME,
                    &js_quote(&profile.username.clone().to_string()),
                ),
                (
                    SIGNAL_AVATAR_SEED,
                    &js_quote(&profile.avatar_seed.clone().unwrap_or_default()),
                ),
                (
                    SIGNAL_FULL_NAME,
                    &js_quote(&profile.full_name.clone().unwrap_or_default()),
                ),
            ])
            .await?;

            FormState::put(ctx, &profile, &form_state).await?;

            Ok(())
        }

        Route::InputtedUsername { .. } => {
            unimplemented!()
        }

        Route::SubmittedForm { .. } => {
            let username = r.payload.get_first(SIGNAL_USERNAME).unwrap_or_default();
            let full_name = r.payload.get_first(SIGNAL_FULL_NAME).unwrap_or_default();
            let avatar_seed = r.payload.get_first(SIGNAL_AVATAR_SEED).unwrap_or_default();

            let user_id = match r.user_id(ctx).await {
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

            ctx.user_profile_db.upsert_one(uow(), &profile_new).await?;

            w.send_toast_dark("Profile updated").await?;

            user::account_screen::redirect_to(ctx, r, w, &r.user_id(ctx).await).await?;

            Ok(())
        }

        Route::ClickedRandomSeed => {
            let avatar_seed_new = random::string(32);

            w.send_signal(SIGNAL_AVATAR_SEED, &js_quote(&avatar_seed_new))
                .await?;

            let profile = ctx
                .user_profile_db
                .find_one_by_user_id(&r.user_id(ctx).await.unwrap_or_default())
                .await?
                .unwrap_or_default();

            let mut form_state = FormState::get(&ctx, &profile).await;

            form_state.avatar_seed.push(avatar_seed_new);

            FormState::put(&ctx, &profile, &form_state).await?;

            Ok(())
        }
    }
}

async fn respond_failed_to_load(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    w.send_toast_dark("User not found. Try logging in again")
        .await?;

    user::account_screen::redirect_to(ctx, r, w, &None).await?;

    Ok(())
}

fn view_screen_root() -> Elem {
    form()
        .class("flex flex-col w-full flex-1 overflow-hidden")
        .data_signal(SIGNAL_USERNAME, "''")
        .data_signal(SIGNAL_AVATAR_SEED, "''")
        .data_signal(SIGNAL_FULL_NAME, "''")
        .debug_signals(false)
        .child(
            TopBar::default()
                .back_url(user::route::Route::AccountScreen.url())
                .title("Edit Profile")
                .view(),
        )
}

fn view_screen_loading() -> Elem {
    view_screen_root().child(spinner_page::view())
}

fn view_screen(profile: UserProfile) -> Elem {
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
                .child(
                    fieldset()
                        .class("flex flex-col w-full gap-4 items-center justify-center")
                        .child(
                            Avatar::default()
                                .data_attributes_src(&format!(
                                    "{}.value.trim().length === 0 ? null : {}",
                                    SIGNAL_AVATAR_SEED,
                                    js_avatar_url(&js_dot_value(SIGNAL_AVATAR_SEED))
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
                                                .icon(|class| icon::rotate_left(&class))
                                                .view(),
                                        )
                                        .child(
                                            IconButton::default()
                                                .label("Redo".to_owned())
                                                .icon(|class| icon::rotate_right(&class))
                                                .view(),
                                        ),
                                )
                                .child(
                                    Button::default().label("Random seed").view().data_on(|b| {
                                        b.click().sse(&Route::ClickedRandomSeed.url())
                                    }),
                                ),
                        ),
                )
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
            BottomBarForm::default()
                .cancel_url(&user::route::Route::AccountScreen.url())
                .view(),
        )
}
