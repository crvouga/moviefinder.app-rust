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

pub async fn respond(
    _ctx: &Ctx,
    _r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen => {
            sse()
                .event_merge_fragments()
                .data_fragments(view_index_login_cta())
                .send(w)
                .await
        }
    }
}

pub fn view_index_login_cta() -> Elem {
    div()
        .id_root()
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(div().class("flex-1").child_text("Account"))
        .child(bottom_bar::view(bottom_bar::Active::Account, &""))
}
