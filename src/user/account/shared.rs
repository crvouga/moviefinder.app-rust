use super::{account_::Account, login_with_sms, route::Route};
use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        ui::{button::Button, icon},
    },
    ctx::Ctx,
    req::Req,
    ui::bottom_bar::BottomBar,
};

pub async fn respond_account_screen(
    _ctx: &Ctx,
    _r: &Req,
    _route: &Route,
    w: &mut ResponseWriter,
    account: &Account,
) -> Result<(), std::io::Error> {
    w.send_screen_frag(view_screen_account()).await?;
    Ok(())
}

pub fn view_screen_account() -> Elem {
    div()
        .id("account")
        .class("w-full flex-1 flex items-center justify-center flex-col")
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
