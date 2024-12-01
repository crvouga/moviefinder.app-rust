use super::route::Route;
use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
    },
    ctx::Ctx,
    req::Req,
};

pub async fn respond(
    _ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::ScreenPhone => {
            sse().send_screen(r, w, "", view_screen_phone()).await?;
            Ok(())
        }

        Route::ScreenCode => {
            sse().send_screen(r, w, "", view_screen_code()).await?;
            Ok(())
        }
    }
}

pub fn view_screen_phone() -> Elem {
    div().child_text("phone")
}

pub fn view_screen_code() -> Elem {
    div().child_text("code")
}
