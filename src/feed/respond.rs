use super::{controls, core::Feed, feed_id::FeedId, feed_item::FeedItem, route::Route};
use crate::{
    core::{
        html::*,
        query::{Filter, Query},
        res::Res,
        ui::{
            self,
            chip::{Chip, ChipSize},
            image::Image,
        },
    },
    ctx::{self, Ctx},
    media::{
        self,
        genre::{genre::Genre, genre_id::GenreId},
        media_id::MediaId,
    },
    req::Req,
    route,
    ui::bottom_bar,
    user_session::session_id::SessionId,
};

pub async fn respond(ctx: &ctx::Ctx, req: &Req, route: &Route) -> Res {
    match route {
        Route::Index => {
            let feed_id = ctx
                .session_feed_mapping_db
                .get(req.session_id.clone())
                .await
                .unwrap_or(None)
                .unwrap_or_default();

            let feed = ctx.feed_db.get_with_fallback(feed_id.clone()).await;

            put_feed(ctx, req.session_id.clone(), &feed).await;

            let model = ViewModel::load(ctx, &feed_id).await;

            view_feed(&model).into()
        }

        Route::ChangedSlide { feed_id } => {
            let slide_index_new = req
                .form_data
                .get_first("feedIndex")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or_default();

            let feed = ctx.feed_db.get_with_fallback(feed_id.clone()).await;

            let feed_new = Feed {
                active_index: slide_index_new,
                ..feed
            };

            put_feed(ctx, req.session_id.clone(), &feed_new).await;

            Res::empty()
        }

        Route::LoadInitial { feed_id } => {
            let feed = ctx.feed_db.get_with_fallback(feed_id.clone()).await;

            let query = Query {
                limit: 3,
                offset: feed.active_index,
                filter: Filter::None,
            };

            let queried = ctx.media_db.query(query).await;

            match queried {
                Err(err) => ui::error::page(&err).into(),

                Ok(paginated) => {
                    let feed_items = paginated
                        .items
                        .into_iter()
                        .enumerate()
                        .map(|(index, media)| FeedItem::from((media, index + feed.active_index)))
                        .collect::<Vec<FeedItem>>();

                    view_feed_items(feed_id, &feed_items).into()
                }
            }
        }

        Route::LoadMore {
            feed_id,
            start_feed_index,
        } => {
            let query = Query {
                limit: 3,
                offset: start_feed_index.clone(),
                filter: Filter::None,
            };

            let queried = ctx.media_db.query(query).await;

            match queried {
                Err(err) => ui::error::page(&err).into(),

                Ok(paginated) => {
                    let feed_items = paginated
                        .items
                        .into_iter()
                        .enumerate()
                        .map(|(index, media)| FeedItem::from((media, index + start_feed_index)))
                        .collect::<Vec<FeedItem>>();

                    view_feed_items(feed_id, &feed_items).into()
                }
            }
        }

        Route::Controls { feed_id, child } => {
            controls::respond::respond(ctx, req, feed_id, child).await
        }
    }
}

async fn put_feed(ctx: &ctx::Ctx, session_id: SessionId, feed: &Feed) {
    ctx.feed_db.put(feed.clone()).await.unwrap_or(());
    ctx.session_feed_mapping_db
        .put(session_id.clone(), feed.feed_id.clone())
        .await
        .unwrap_or(());
}

struct ViewModel {
    feed: Feed,
    genres: Vec<Genre>,
}

impl ViewModel {
    async fn load(ctx: &Ctx, feed_id: &FeedId) -> Self {
        let feed = ctx.feed_db.get_with_fallback(feed_id.clone()).await;

        let genres = ctx.genre_db.get_all().await.unwrap_or(vec![]);

        Self { feed, genres }
    }
}

fn view_top_bar(model: &ViewModel) -> Elem {
    button()
        .class(
            "w-full h-16 border-b flex items-center justify-center relative pl-2 overflow-hidden",
        )
        .root_push(route::Route::Feed(Route::Controls {
            feed_id: model.feed.feed_id.clone(),
            child: controls::route::Route::Index,
        }))
        .child(view_chips(&model))
        .child(view_open_controls_button())
}

fn view_open_controls_button() -> Elem {
    div()
        .class("absolute top-0 right-0 h-full flex items-center justify-center")
        .child(div().class("w-16 h-full from-transparent to-black bg-gradient-to-r"))
        .child(
            div()
                .class("size-16 bg-black flex items-center justify-center")
                .child(ui::icon::adjustments_vertical("size-8")),
        )
}

fn view_chips(model: &ViewModel) -> Elem {
    div()
        .class("flex flex-row gap-2 p-2 flex-1 overflow-hidden")
        .children(
            model
                .feed
                .genre_ids
                .iter()
                .map(|genre_id| view_genre_chip(model, genre_id))
                .collect::<Vec<Elem>>(),
        )
}

fn view_genre_chip(model: &ViewModel, genre_id: &GenreId) -> Elem {
    let maybe_genre = model
        .genres
        .clone()
        .into_iter()
        .find(|g| g.id.clone() == genre_id.clone());

    match maybe_genre {
        Some(genre) => Chip::default()
            .id(&genre.id.to_string())
            .label(&genre.name)
            .checked(true)
            .disabled(true)
            .size(ChipSize::Medium)
            .view(),
        None => frag(),
    }
}

fn view_feed_root() -> Elem {
    div().class("w-full flex-1 flex items-center justify-center flex-col overflow-hidden")
}

fn view_feed(model: &ViewModel) -> Elem {
    view_feed_root()
        .child(view_top_bar(&model))
        .child(view_swiper(&model.feed.feed_id))
        .child(view_bottom_bar())
}

fn view_bottom_bar() -> Elem {
    bottom_bar::view(bottom_bar::Active::Home)
}

fn view_swiper(feed_id: &FeedId) -> Elem {
    ui::swiper::container()
    .swiper_direction_vertical()
    .swiper_slides_per_view("1")
    .class("flex-1 flex flex-col w-full items-center justify-center overflow-hidden")
    .hx_trigger_custom("swiperslidechange from:swiper-container")
    .hx_swap_none()
    .hx_post(
        route::Route::Feed(Route::ChangedSlide {
            feed_id: feed_id.clone(),
        })
        .encode()
        .as_str(),
    )
    .hx_vals(
        r#"
        js:{
            feedIndex: parseInt(event?.detail?.[0]?.slides?.[event?.detail?.[0]?.activeIndex]?.getAttribute?.('data-feed-index'), 10)
        }
        "#
    )
    .child(view_load_initial(feed_id))
}

fn view_feed_items(feed_id: &FeedId, feed_items: &[FeedItem]) -> Elem {
    frag()
        .children(feed_items.iter().map(view_feed_item).collect::<Vec<Elem>>())
        .child(
            feed_items
                .iter()
                .last()
                .map(|feed_item| view_load_more(feed_id, feed_item.to_feed_index() + 1))
                .unwrap_or(frag()),
        )
}

fn view_load_more(feed_id: &FeedId, start_feed_index: usize) -> Elem {
    ui::swiper::slide()
        .class("flex-1 flex flex-col items-center justify-center")
        .hx_get(
            &route::Route::Feed(Route::LoadMore {
                feed_id: feed_id.clone(),
                start_feed_index,
            })
            .encode(),
        )
        .hx_trigger_intersect()
        .hx_swap_outer_html()
        .child(Image::view().src(" ").class("w-full h-full object-cover"))
}

fn to_media_details_route(media_id: &MediaId) -> route::Route {
    route::Route::Media(media::route::Route::Details(
        media::details::route::Route::Index {
            media_id: media_id.clone(),
        },
    ))
}

fn view_feed_item(feed_item: &FeedItem) -> Elem {
    ui::swiper::slide()
        .class("w-full h-full flex flex-col items-center justify-center cursor-pointer relative")
        .attr("data-feed-index", &feed_item.to_feed_index().to_string())
        .child(view_feed_item_content(feed_item))
}

fn view_feed_item_content(feed_item: &FeedItem) -> Elem {
    match feed_item {
        FeedItem::Media {
            media,
            feed_index: _,
        } => button()
            .class("w-full h-full")
            .root_push(to_media_details_route(&media.media_id))
            .child(
                Image::view()
                    .class("w-full h-full object-cover")
                    .width("100%")
                    .height("100%")
                    .src(media.media_poster.to_highest_res()),
            ),
    }
}

fn view_load_initial(feed_id: &FeedId) -> Elem {
    div()
        .class("flex-1 flex flex-col items-center justify-center")
        .hx_get(
            &route::Route::Feed(Route::LoadInitial {
                feed_id: feed_id.clone(),
            })
            .encode(),
        )
        .hx_trigger_load()
        .hx_swap_outer_html()
        .child(Image::view().src(" ").class("w-full h-full object-cover"))
}
