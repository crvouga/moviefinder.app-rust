use super::{login_with_sms, route::Route};
use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        ui::{button::Button, icon},
    },
    ctx::Ctx,
    req::Req,
    route,
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
        Route::Screen => {
            w.send_screen_frag(view_screen_login_cta()).await?;
            Ok(())
        }
    }
}

impl Route {
    pub fn route(self) -> route::Route {
        route::Route::User(self)
    }

    pub fn url(self) -> String {
        self.route().encode()
    }
}

pub fn view_screen_login_cta() -> Elem {
    div()
        .id("login-cta")
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
