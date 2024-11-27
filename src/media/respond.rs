use crate::{core::http::response_writer::HttpResponseWriter, ctx::Ctx, res::Res};

use super::{details, route::Route};

pub async fn respond(response_writer: &mut HttpResponseWriter, ctx: &Ctx, route: &Route) -> Res {
    match route {
        Route::Details(route) => details::respond::respond(response_writer, ctx, route).await,
    }
}
