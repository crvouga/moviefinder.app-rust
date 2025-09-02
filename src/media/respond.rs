use super::{
    details,
    interaction::{interaction_form, interaction_list},
    route::Route,
};
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::CoreError> {
    match route {
        Route::Details(route) => details::respond::respond(ctx, r, route, w).await,
        Route::InteractionForm(route) => interaction_form::respond::respond(ctx, r, route, w).await,
        Route::InteractionList(route) => interaction_list::respond::respond(ctx, r, route, w).await,
    }
}
