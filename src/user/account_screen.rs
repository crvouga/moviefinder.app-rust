use super::{
    login_with_sms, logout, route::Route, user_account::user_account_::UserAccount,
    user_id::UserId, user_profile::user_profile_::UserProfile,
};
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

pub async fn respond_screen(
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

fn view_loading_screen() -> Elem {
    div()
        .id("loading")
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(TopBar::default().title("Account").view())
        .child(spinner_page::view())
        .child(BottomBar::default().active_account().view())
}

async fn respond_screen_logged_out(w: &mut ResponseWriter) -> Result<(), std::io::Error> {
    w.send_screen(view_logged_out()).await?;
    Ok(())
}

fn view_logged_out() -> Elem {
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
                        .id("logout")
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

impl Route {
    fn route(self) -> crate::route::Route {
        crate::route::Route::User(self)
    }

    fn url(self) -> String {
        self.route().url()
    }
}

fn view_logged_in(_account: &UserAccount, _profile: &UserProfile) -> Elem {
    div()
        .id("account")
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(TopBar::default().title("Account").view())
        .child(
            div()
                .class("flex-1 flex items-center justify-center flex-col gap-3")
                .child(
                    Button::default()
                        .color_primary()
                        .label("Logout")
                        .view()
                        .data_on(|b| {
                            b.click()
                                .get(&Route::Logout(logout::route::Route::Drawer).url())
                        }),
                ),
        )
        .child(BottomBar::default().active_account().view())
}
