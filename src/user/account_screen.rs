use children::text;

use super::{
    edit_profile, login_with_sms, logout, route::Route, user_account::user_account_::UserAccount,
    user_id::UserId, user_profile::user_profile_::UserProfile,
};
use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        js::Js,
        ui::{avatar::Avatar, button::Button, icon, spinner_page, top_bar::TopBar},
    },
    ctx::Ctx,
    media::interaction::interaction_list,
    req::Req,
    ui::{bottom_bar::BottomBar, route::AppRoute},
};

pub async fn redirect_to(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
    user_id: &Option<UserId>,
) -> Result<(), std::io::Error> {
    w.send_script(&Js::push_url(&Route::AccountScreen.url()))
        .await?;

    respond(ctx, r, w, user_id).await?;

    Ok(())
}

pub async fn respond(
    ctx: &Ctx,
    _r: &Req,
    w: &mut ResponseWriter,
    user_id: &Option<UserId>,
) -> Result<(), std::io::Error> {
    match user_id {
        None => {
            respond_screen_logged_out(w).await?;

            Ok(())
        }

        Some(user_id) => {
            w.send_screen(view_loading_screen()).await?;

            let maybe_account = ctx.user_account_db.find_one_by_user_id(&user_id).await?;

            let account = match maybe_account {
                Some(account) => account,
                None => {
                    w.send_toast_dark("Account not found. Try logging in again")
                        .await?;
                    return respond_screen_logged_out(w).await;
                }
            };

            let maybe_profile = ctx.user_profile_db.find_one_by_user_id(&user_id).await?;

            let profile = match maybe_profile {
                Some(profile) => profile,
                None => {
                    w.send_toast_dark("Profile not found. Try logging in again")
                        .await?;
                    return respond_screen_logged_out(w).await;
                }
            };

            w.send_screen(view_logged_in(&account, &profile)).await?;

            Ok(())
        }
    }
}

async fn respond_screen_logged_out(w: &mut ResponseWriter) -> Result<(), std::io::Error> {
    w.send_screen(view_logged_out()).await?;
    Ok(())
}

fn view_loading_screen() -> Elem {
    div()
        .id("loading")
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(TopBar::default().title("Account").view())
        .child(spinner_page::view())
        .child(BottomBar::default().active_account().view())
}

fn view_logged_out() -> Elem {
    div()
        .id("login-cta")
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(TopBar::default().title("Account").view())
        .child(
            div()
                .class("flex-1 flex items-center justify-center flex-col gap-3")
                .child(icon::solid::door_open("size-24"))
                .child(
                    div()
                        .class("text-2xl font-bold text-center")
                        .child_text("Login to access your account"),
                )
                .child(
                    Button::default()
                        .label("Login")
                        .color_primary()
                        .view()
                        .id("login-button")
                        .data_on(|b| {
                            b.press_down()
                                .push_then_sse(&login_with_sms::route::Route::ScreenPhone.url())
                        }),
                ),
        )
        .child(BottomBar::default().active_account().view())
}

fn view_logged_in(_account: &UserAccount, profile: &UserProfile) -> Elem {
    div()
        .id("account")
        .class("w-full h-full flex-1 flex items-center justify-center flex-col")
        .child(TopBar::default().title("Account").view())
        .child(view_profile_header(_account, profile))
        .child(
            div()
                .class("w-full flex-1")
                .child(interaction_list::respond::view_lists_section(
                    profile.user_id.clone(),
                )),
        )
        .child(BottomBar::default().active_account().view())
}

fn view_profile_header(_account: &UserAccount, profile: &UserProfile) -> Elem {
    div()
        .class("flex items-center justify-start flex-col gap-6 p-6 w-full")
        .child(
            div()
                .class("w-full flex flex-col items-center gap-4")
                .child(
                    Avatar::default()
                        .data_attributes_src(&Js::str(&profile.to_avatar_url()))
                        .class("size-24")
                        .view(),
                )
                .child(
                    p().child(text(&profile.username.to_at_username()))
                        .class("text-xl font-base w-full text-center"),
                ),
        )
        .child(
            div()
                .class("flex w-full gap-3 justify-center")
                .child(
                    edit_profile::respond::view_open_edit_profile_screen_button(
                        profile.user_id.clone(),
                    )
                    .color_gray()
                    .size_small()
                    .view()
                    .class("w-full max-w-[200px]"),
                )
                .child(
                    logout::respond::view_open_logout_drawer_button()
                        .color_gray()
                        .size_small()
                        .view()
                        .class("w-full max-w-[200px]"),
                ),
        )
}
