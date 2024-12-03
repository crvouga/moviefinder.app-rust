use super::{login_with_sms, logout, route::Route, shared::respond_account_screen};
use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        ui::{button::Button, icon, spinner_page, top_bar::TopBar},
    },
    ctx::Ctx,
    req::Req,
    ui::bottom_bar::BottomBar,
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::LoginWithSms(child) => login_with_sms::respond::respond(ctx, r, child, w).await,
        Route::Logout(child) => logout::respond::respond(ctx, r, child, w).await,
        Route::Screen => match &r.user_id {
            Some(user_id) => {
                w.send_screen_frag(view_loading_screen()).await?;

                let maybe_account = ctx.user_account_db.find_one_by_user_id(&user_id).await?;

                let account = match maybe_account {
                    Some(account) => account,
                    None => {
                        w.send_toast_dark("Account not found. Try logging in again")
                            .await?;
                        return respond_screen_login_cta(w).await;
                    }
                };

                let maybe_profile = ctx.user_profile_db.find_one_by_user_id(&user_id).await?;

                let profile = match maybe_profile {
                    Some(profile) => profile,
                    None => {
                        w.send_toast_dark("Profile not found. Try logging in again")
                            .await?;
                        return respond_screen_login_cta(w).await;
                    }
                };

                respond_account_screen(ctx, r, w, &account, &profile).await?;

                Ok(())
            }
            None => {
                respond_screen_login_cta(w).await?;
                Ok(())
            }
        },
    }
}

async fn respond_screen_login_cta(w: &mut ResponseWriter) -> Result<(), std::io::Error> {
    w.send_screen_frag(view_screen_login_cta()).await?;
    Ok(())
}

pub fn view_loading_screen() -> Elem {
    div()
        .id("loading")
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(TopBar::default().title("Account").view())
        .child(spinner_page::view())
        .child(BottomBar::default().active_account().view())
}

pub fn view_screen_login_cta() -> Elem {
    div()
        .id("login-cta")
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(TopBar::default().title("Account").view())
        .child(
            div()
                .class("flex-1 flex items-center justify-center flex-col gap-3")
                .child(icon::door_open("size-24"))
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
                        .data_on(|b| {
                            b.click().push_then_get(
                                &Route::LoginWithSms(login_with_sms::route::Route::ScreenPhone)
                                    .url(),
                            )
                        }),
                ),
        )
        .child(BottomBar::default().active_account().view())
}
