use super::route::Route;
use crate::{
    core::{
        html::*,
        res::Res,
        ui::{
            self,
            button::{Button, Color},
            chip::Chip,
        },
    },
    ctx::Ctx,
    feed::{self, core::Feed, feed_id::FeedId},
    media::genre::{genre::Genre, genre_id::GenreId},
    req::Req,
    route,
    ui::{root::ROOT_SELECTOR, top_bar},
};

#[derive(Debug)]
struct ViewModel {
    feed: Feed,
    genres: Vec<Genre>,
}

const BACK_ROUTE: route::Route = route::Route::Feed(feed::route::Route::Index);
const GENRE_ID_KEY: &str = "genre_id";

pub async fn respond(ctx: &Ctx, req: &Req, feed_id: &FeedId, route: &Route) -> Res {
    match route {
        Route::Index => {
            let feed = ctx.feed_db.get_with_fallback(feed_id.clone()).await;

            let genres = ctx.genre_db.get_all().await.unwrap_or(vec![]);

            let view_model = ViewModel { feed, genres };

            view_controls(&view_model).into()
        }

        Route::ClickedSave => {
            let genre_ids_new: Vec<GenreId> = req
                .form_data
                .get_all(GENRE_ID_KEY)
                .cloned()
                .unwrap_or(vec![])
                .into_iter()
                .map(GenreId::new)
                .collect();

            let feed = ctx.feed_db.get_with_fallback(feed_id.clone()).await;

            let feed_new = Feed {
                genre_ids: genre_ids_new,
                active_index: 0,
                ..feed
            };

            ctx.feed_db.put(feed_new.clone()).await.unwrap_or(());

            Res::redirect(
                route::Route::Feed(feed::route::Route::Index)
                    .encode()
                    .to_string(),
                ROOT_SELECTOR.to_string(),
            )
        }
    }
}

fn view_controls(view_model: &ViewModel) -> Elem {
    form()
        .class("w-full h-full flex flex-col overflow-hidden relative")
        .hx_post(
            &route::Route::Feed(feed::route::Route::Controls {
                feed_id: view_model.feed.feed_id.clone(),
                child: Route::ClickedSave,
            })
            .encode(),
        )
        .hx_swap_none()
        .child(view_top_bar())
        .child(view_form(view_model))
        .child(view_bottom_bar())
}

fn view_top_bar() -> Elem {
    div()
        .class("absolute right-0 top-0")
        .child(top_bar::CancelButton::view(BACK_ROUTE))
}

fn view_bottom_bar() -> Elem {
    div()
        .class("flex-none flex flex-row items-center justify-center p-4 border-t gap-4")
        .child(
            Button::new()
                .label("Cancel")
                .color(Color::Gray)
                .view()
                .hx_get(&BACK_ROUTE.encode())
                .hx_push_url()
                .hx_preload_mouse_down()
                .hx_swap_inner_html()
                .hx_target(ROOT_SELECTOR)
                .type_("button")
                .class("flex-1"),
        )
        .child(
            Button::new()
                .label("Save")
                .color(ui::button::Color::Primary)
                .view()
                .type_("submit")
                .class("flex-1"),
        )
}

fn view_form(view_model: &ViewModel) -> Elem {
    div()
        .class("flex-1 flex flex-col py-8 px-6 overflow-y-auto")
        .child(view_section("Genres", vec![view_genre_chips(&view_model)]))
}

fn view_section(title: &str, children: Vec<Elem>) -> Elem {
    div()
        .class("flex flex-col gap-4")
        .child(view_section_title(title))
        .children(&children)
}

fn view_section_title(title: &str) -> Elem {
    div().class("text-4xl font-bold").child_text(title)
}

fn view_genre_chips(view_model: &ViewModel) -> Elem {
    div()
        .class("flex-1 flex flex-row items-center flex-wrap gap-2")
        .children(
            &view_model
                .genres
                .iter()
                .map(|genre| view_genre_chip(view_model, genre))
                .collect::<Vec<Elem>>(),
        )
}

fn view_genre_chip(view_model: &ViewModel, genre: &Genre) -> Elem {
    Chip::default()
        .id(genre.id.as_str())
        .label(&genre.name)
        .name(GENRE_ID_KEY)
        .checked(is_genre_checked(view_model, genre))
        .size(ui::chip::ChipSize::Large)
        .view()
}

fn is_genre_checked(view_model: &ViewModel, genre: &Genre) -> bool {
    let checked = view_model
        .feed
        .genre_ids
        .iter()
        .any(|genre_id| genre_id.clone() == genre.id);

    checked
}
