use crate::{
    core::{
        html::*,
        hx,
        query::{Filter, Op, Query},
        res::Res,
        ui,
    },
    ctx::Ctx,
    feed,
    media::{self, media::Media, media_db::Field, media_id::MediaId},
    route,
    ui::{root::ROOT_SELECTOR, top_bar},
};

use super::route::Route;

pub async fn respond(route: &Route, ctx: &Ctx) -> Res {
    match route {
        Route::Index { media_id } => Res::Html(view_load(media_id)),

        Route::Load { media_id } => {
            // sleep
            // tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let query = Query {
                limit: 1,
                offset: 0,
                filter: Filter::clause(Field::MediaId, Op::Eq, media_id.as_str().to_string()),
            };

            let queried = ctx.media_db.query(query).await;

            let result = match queried {
                Ok(result) => result,
                Err(err) => return ui::error::page(&err).into(),
            };

            let media = match result.items.into_iter().next() {
                Some(media) => media,
                None => return ui::error::page("Media not found").into(),
            };

            view_details(&media).into()
        }
    }
}

fn view_layout(media: Option<Media>, attrs: &[Attr], children: &[Elem]) -> Elem {
    let top_bar_title = media.as_ref().map_or("", |m| &m.media_title);

    let image_src = media
        .as_ref()
        .map_or(" ", |m| m.media_backdrop.to_highest_res());

    div(
        &[
            class("flex flex-col"),
            // Additional attributes passed in
        ]
        .iter()
        .chain(attrs)
        .cloned()
        .collect::<Vec<_>>()
        .as_slice(),
        &[
            top_bar::view(route::Route::Feed(feed::route::Route::Index), top_bar_title),
            div(
                &[class("flex flex-col gap-6 items-center")],
                &[div(
                    &[class("w-full aspect-video overflow-hidden border-b")],
                    &[ui::image::view(&[
                        class("w-full h-full select-none"),
                        src(image_src),
                    ])],
                )]
                .iter()
                .chain(children)
                .cloned()
                .collect::<Vec<_>>()
                .as_slice(),
            ),
        ],
    )
}

fn view_load(media_id: &MediaId) -> Elem {
    view_layout(
        None,
        &[
            hx::Trigger::Load.attr(),
            hx::Swap::InnerHtml.attr(),
            hx::target(&ROOT_SELECTOR),
            hx::get(&load_route(media_id).encode()),
        ],
        &[ui::icon::spinner(&[class("animate-spin size-16")])],
    )
}

fn view_details(media: &Media) -> Elem {
    view_layout(
        Some(media.clone()),
        &[],
        &[div(
            &[class("flex flex-col gap-4 items-center")],
            &[view_title(media), view_description(media)],
        )],
    )
}

fn view_title(media: &Media) -> Elem {
    p(
        &[class("text-3xl font-bold text-center px-6")],
        &[text(&media.media_title)],
    )
}

fn view_description(media: &Media) -> Elem {
    p(
        &[class("text-base text-opacity font-normal text-center px-6")],
        &[text(&media.media_description)],
    )
}

fn load_route(media_id: &MediaId) -> route::Route {
    route::Route::Media(media::route::Route::Details(
        media::details::route::Route::Load {
            media_id: media_id.clone(),
        },
    ))
}
