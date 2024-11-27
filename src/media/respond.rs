use crate::{
    core::http::{request::Request, response_writer::ResponseWriter},
    ctx::Ctx,
};

use super::{details, route::Route};

pub async fn respond(
    ctx: &Ctx,
    r: &Request,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Details(route) => details::respond::respond(ctx, r, route, w).await,
    }
}
