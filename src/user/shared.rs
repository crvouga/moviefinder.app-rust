use children::text;

use crate::{
    core::{html::*, http::response_writer::ResponseWriter},
    ctx::Ctx,
    req::Req,
    ui::bottom_bar::BottomBar,
    user::profile::profile_::UserProfile,
};

use super::account::account_::UserAccount;

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

pub fn view_screen_account(_account: &UserAccount, _profile: &UserProfile) -> Elem {
    div()
        .id("account")
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(
            div()
                .class("flex-1 flex items-center justify-center flex-col gap-3")
                .child(text("Account")),
        )
        .child(BottomBar::default().active_account().view())
}
