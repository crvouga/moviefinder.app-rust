use super::route::Route;
use crate::{
    core::{html::*, res::Res, ui},
    ctx::Ctx,
    feed::{self, core::Feed, feed_id::FeedId},
    media::genre::genre::Genre,
    req::Req,
    route,
    ui::top_bar,
};

struct ViewModel {
    feed: Feed,
    genres: Vec<Genre>,
}

pub async fn respond(ctx: &Ctx, _req: &Req, feed_id: &FeedId, route: &Route) -> Res {
    match route {
        Route::Index => {
            let feed = ctx.feed_db.get_with_fallback(feed_id.clone()).await;

            let genres = ctx.genre_db.get_all().await.unwrap_or(vec![]);

            let view_model = ViewModel { feed, genres };

            view_controls(&view_model).into()
        }
    }
}

fn view_controls(view_model: &ViewModel) -> Elem {
    div(
        &[class("w-full h-full flex flex-col overflow-hidden")],
        &[view_top_bar(), view_form(view_model)],
    )
}

fn view_top_bar() -> Elem {
    top_bar::root(
        &[],
        &[
            top_bar::empty(),
            top_bar::title("Controls"),
            top_bar::cancel_button(route::Route::Feed(feed::route::Route::Index)),
        ],
    )
}

fn view_form(view_model: &ViewModel) -> Elem {
    form(
        &[class("flex-1 flex flex-col py-4 px-6 overflow-y-auto")],
        &[
            //
            view_section("Genres", vec![view_genre_chips(&view_model)]),
        ],
    )
}

fn view_section(title: &str, children: Vec<Elem>) -> Elem {
    div(
        &[class("flex flex-col gap-4")],
        [view_section_title(title)]
            .into_iter()
            .chain(children.into_iter())
            .collect::<Vec<Elem>>()
            .as_ref(),
    )
}

fn view_section_title(title: &str) -> Elem {
    div(&[class("text-2xl font-bold")], &[text(title)])
}

fn view_genre_chips(view_model: &ViewModel) -> Elem {
    div(
        &[class("flex-1 flex flex-row items-center flex-wrap gap-2")],
        &view_model
            .genres
            .iter()
            .map(|genre| view_genre_chip(genre))
            .collect::<Vec<Elem>>(),
    )
}

fn view_genre_chip(genre: &Genre) -> Elem {
    ui::chip::view(genre.id.as_str(), genre.name.as_str())
}
