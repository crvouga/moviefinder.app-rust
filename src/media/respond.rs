use crate::{ctx::Ctx, res::Res};

use super::{details, route::Route};

pub async fn respond(route: Route, ctx: &Ctx) -> Res {
    match route {
        Route::Details(child) => details::respond::respond(child, ctx).await,
    }
}
