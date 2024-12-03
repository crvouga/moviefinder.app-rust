use super::{
    logout, route::Route, user_account::user_account_::UserAccount,
    user_profile::user_profile_::UserProfile,
};
use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        ui::{button::Button, top_bar::TopBar},
    },
    ctx::Ctx,
    req::Req,
    ui::bottom_bar::BottomBar,
};
use children::text;

pub async fn respond_account_screen(
    _ctx: &Ctx,
    _r: &Req,
    w: &mut ResponseWriter,
    account: &UserAccount,
    profile: &UserProfile,
) -> Result<(), std::io::Error> {
    w.send_screen_frag(view_screen_account(account, profile))
        .await?;
    Ok(())
}

impl Route {
    pub fn route(self) -> crate::route::Route {
        crate::route::Route::User(self)
    }

    pub fn url(self) -> String {
        self.route().encode()
    }
}

pub fn view_screen_account(_account: &UserAccount, _profile: &UserProfile) -> Elem {
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
