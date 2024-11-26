use super::route::Route;
use crate::{
    core::{html::*, http::response_writer::HttpResponseWriter},
    res::Res,
    ui::{bottom_bar, root::ROOT_ID},
};

pub async fn respond(response_writer: &mut HttpResponseWriter, route: &Route) -> Res {
    match route {
        Route::Index => {
            response_writer.merge_fragment(view_index_login_cta()).await;

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
