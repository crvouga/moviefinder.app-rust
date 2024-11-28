use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

use super::{details, route::Route};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Details(route) => details::respond::respond(ctx, r, route, w).await,
    }
}
