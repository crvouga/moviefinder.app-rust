use super::{login_with_sms, route::Route};
use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        ui::{button::Button, icon},
    },
    ctx::Ctx,
    req::Req,
    route,
    ui::bottom_bar,
};

fn to_screen_id(child_id: &str) -> String {
    return child_id.to_string();
    // let child_id = child_id.trim();
    // let prefix = "account";

    // if child_id.is_empty() {
    //     prefix.to_string()
    // } else {
    //     format!("{}-{}", prefix, child_id)
    // }
}

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::LoginWithSms(child) => login_with_sms::respond::respond(ctx, r, child, w).await,
        Route::Screen => {
            w.send_screen_frag(view_index_login_cta()).await?;
            Ok(())
        }
    }
}

fn route(route: Route) -> String {
    route::Route::Account(route).encode()
}

pub fn view_index_login_cta() -> Elem {
    div()
        .id(&to_screen_id(""))
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
                    Button::new()
                        .label("Login")
                        .color_primary()
                        .view()
                        .data_on(|b| {
                            b.click().push_then_get(&route(Route::LoginWithSms(
                                login_with_sms::route::Route::ScreenPhone,
                            )))
                        }),
                ),
        )
        .child(bottom_bar::view(bottom_bar::Active::Account))
}
