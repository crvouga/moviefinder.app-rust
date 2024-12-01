use super::route::Route;
use crate::{
    account,
    core::{html::*, http::response_writer::ResponseWriter},
    ctx::Ctx,
    req::Req,
    route,
    ui::top_bar::TopBar,
};

pub async fn respond(
    _ctx: &Ctx,
    _r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::ScreenPhone => {
            w.send_screen_frag(view_screen_phone()).await?;

            Ok(())
        }

        Route::ScreenCode => {
            w.send_screen_frag(view_screen_code()).await?;

            Ok(())
        }
    }
}

pub fn view_screen_phone() -> Elem {
    div()
        .id("phone")
        .class("w-full h-full flex flex-col")
        .data_store("{ phone: '' }")
        .child(
            TopBar::default()
                .title("Login with phone")
                .back_button(route::Route::Account(account::route::Route::Screen))
                .view(),
        )
        .child(
            div().class("flex-1 p-4"), // .child(TextField::)
        )
}

pub fn view_screen_code() -> Elem {
    div().child_text("code")
}
