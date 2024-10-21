use crate::{ctx::Ctx, html::text, res::Res};

use super::route::Route;

pub async fn respond(route: Route, _ctx: &Ctx) -> Res {
    match route {
        Route::Index { media_id } => Res::Html(text(&format!("Index: {}", media_id.as_str()))),
        Route::Load { media_id } => Res::Html(text(&format!("Load: {}", media_id.as_str()))),
    }
}
