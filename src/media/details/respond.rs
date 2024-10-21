use crate::{ctx::Ctx, res::Res};

use super::route::Route;

pub async fn respond(route: Route, _ctx: &Ctx) -> Res {
    match route {
        Route::Index { media_id } => Res::Html(format!("Index: {}", media_id.as_str())),
        Route::Load { media_id } => Res::Html(format!("Load: {}", media_id.as_str())),
    }
}
