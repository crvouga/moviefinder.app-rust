use crate::{
    app::root::ROOT_SELECTOR,
    core::query::{Filter, Operator, Query},
    ctx::Ctx,
    html::*,
    hx,
    media::{self, media::Media, media_db::Field, media_id::MediaId},
    res::Res,
    route, ui,
};

use super::route::Route;

pub async fn respond(route: Route, ctx: &Ctx) -> Res {
    match route {
        Route::Index { media_id } => Res::Html(view_load(media_id)),

        Route::Load { media_id } => {
            let query = Query {
                limit: 1,
                offset: 0,
                filter: Filter::clause(Field::MediaId, Operator::Eq, media_id.as_str().to_string()),
            };

            ctx.media_db
                .query(&query)
                .await
                .map_or_else(
                    |err| ui::error::page(&err),
                    |paginated| {
                        paginated
                            .items
                            .into_iter()
                            .next()
                            .map_or(ui::error::page("Media not found"), |media| {
                                view_details(media)
                            })
                    },
                )
                .into()
        }
    }
}

fn view_load(media_id: MediaId) -> Elem {
    ui::loading::page(&[
        hx::Trigger::Load.attr(),
        hx::Swap::InnerHtml.attr(),
        hx::target(&ROOT_SELECTOR),
        hx::get(&load_route(media_id).encode()),
    ])
}

fn view_details(_media: Media) -> Elem {
    div(&[], &[])
}

fn load_route(media_id: MediaId) -> route::Route {
    route::Route::Media(media::route::Route::Details(
        media::details::route::Route::Load { media_id },
    ))
}
