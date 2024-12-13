use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

use super::{details, interaction::interaction_form, route::Route};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Details(route) => details::respond::respond(ctx, r, route, w).await,
        Route::InteractionForm(route) => interaction_form::respond::respond(ctx, r, route, w).await,
    }
}
