use super::route::Route;
use crate::{
    core::{
        html::*,
        http::{response_writer::HttpResponseWriter, server_sent_event::sse},
    },
    res::Res,
    ui::{bottom_bar, root::ROOT_ID},
};

pub async fn respond(response_writer: &mut HttpResponseWriter, route: &Route) -> Res {
    match route {
        Route::Index => {
            sse()
                .event_merge_fragments()
                .data_fragments(view_index_login_cta())
                .send(response_writer)
                .await;

            Res::empty()
        }
    }
}

pub fn view_index_login_cta() -> Elem {
    div()
        .id(ROOT_ID)
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(div().class("flex-1").child_text("Account"))
        .child(bottom_bar::view(bottom_bar::Active::Account, &""))
}
