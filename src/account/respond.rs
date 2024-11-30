use super::route::Route;
use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
    },
    ctx::Ctx,
    req::Req,
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
    _ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen => {
            sse()
                .send_screen(r, w, &to_screen_id(""), view_index_login_cta())
                .await?;
            Ok(())
        }
    }
}

pub fn view_index_login_cta() -> Elem {
    div()
        .id(&to_screen_id(""))
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(div().class("flex-1").child_text("Account"))
        .child(bottom_bar::view(bottom_bar::Active::Account))
}
