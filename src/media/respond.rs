use crate::{ctx::Ctx, res::Res};

use super::{details, route::Route};

pub async fn respond(ctx: &Ctx, route: &Route) -> Res {
    match route {
        Route::Details(route) => details::respond::respond(ctx, route).await,
    }
}
