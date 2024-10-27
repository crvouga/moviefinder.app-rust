use super::{controls, core::Feed, feed_id::FeedId, feed_item::FeedItem, route::Route};
use crate::{
    core::{
        html::*,
        hx,
        query::{Filter, Query},
        res::Res,
        ui,
    },
    ctx::{self, Ctx},
    media::{
        self,
        genre::{genre::Genre, genre_id::GenreId},
        media_id::MediaId,
    },
    req::Req,
    route,
    ui::{bottom_bar, root::ROOT_SELECTOR, top_bar},
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
    top_bar::root(
        &[],
        &[div(
            &[class(
                "w-full h-full flex items-center justify-center relative pl-2",
            )],
            &[
                view_chips(&model),
                div(
                    &[class(
                        "absolute top-0 right-0 h-full flex items-center justify-center",
                    )],
                    &[
                        div(
                            &[class(
                                "w-16 h-full from-transparent to-black bg-gradient-to-r",
                            )],
                            &[],
                        ),
                        div(
                            &[class("h-full bg-black")],
                            &[view_open_controls_button(&model)],
                        ),
                    ],
                ),
            ],
        )],
    )
}

fn view_open_controls_button(model: &ViewModel) -> Elem {
    button(
        &[
            class("size-16 flex items-center justify-center"),
            hx::Swap::InnerHtml.into(),
            hx::target(ROOT_SELECTOR),
            hx::push_url("true"),
            hx::get(
                route::Route::Feed(Route::Controls {
                    feed_id: model.feed.feed_id.clone(),
                    child: controls::route::Route::Index,
                })
                .encode()
                .as_str(),
            ),
        ],
        &[ui::icon::adjustments_vertical(&[class("size-8")])],
    )
}

fn view_chips(model: &ViewModel) -> Elem {
    div(
        &[class("flex flex-row gap-2 p-2 flex-1 overflow-hidden")],
        model
            .feed
            .genre_ids
            .iter()
            .map(|genre_id| view_genre_chip(model, genre_id))
            .collect::<Vec<Elem>>()
            .as_ref(),
    )
}

fn view_genre_chip(model: &ViewModel, genre_id: &GenreId) -> Elem {
    let maybe_genre = model
        .genres
        .clone()
        .into_iter()
        .find(|g| g.id.clone() == genre_id.clone());

    match maybe_genre {
        Some(genre) => ui::chip::view(
            ui::chip::Props {
                id: genre.id.to_string(),
                label: genre.name.clone(),
                name: "".to_string(),
                checked: true,
                disabled: true,
            },
            &[],
        ),
        None => fragment(&[]),
    }
}

fn view_feed(model: &ViewModel) -> Elem {
    div(
        &[class(
            "w-full flex-1 flex items-center justify-center flex-col overflow-hidden",
        )],
        &[
            view_top_bar(model),
            ui::swiper::container(
                &[
                    class(
                        "flex-1 flex flex-col w-full items-center justify-center overflow-hidden",
                    ),
                    ui::swiper::Direction::Vertical.into(),
                    ui::swiper::slides_per_view("1"),
                    hx::Trigger::Custom("swiperslidechange from:swiper-container".to_string())
                        .into(),
                    hx::Swap::None.into(),
                    hx::post(
                        route::Route::Feed(Route::ChangedSlide {
                            feed_id: model.feed.feed_id.clone(),
                        })
                        .encode()
                        .as_str(),
                    ),
                    hx::vals(
                        r#"
                        js:{
                            feedIndex: parseInt(event?.detail?.[0]?.slides?.[event?.detail?.[0]?.activeIndex]?.getAttribute?.('data-feed-index'), 10)
                        }
                        "#,
                    ),
                ],
                &[view_load_initial(model)],
            ),
            bottom_bar::view(bottom_bar::Active::Home),
        ],
    )
}

fn view_feed_items(feed_id: &FeedId, feed_items: &[FeedItem]) -> Elem {
    let load_more = feed_items
        .iter()
        .last()
        .map(|feed_item| view_load_more(feed_id, feed_item.to_feed_index() + 1))
        .unwrap_or(fragment(&[]));

    fragment(&[
        fragment(&feed_items.iter().map(view_feed_item).collect::<Vec<Elem>>()),
        load_more,
    ])
}

fn view_load_more(feed_id: &FeedId, start_feed_index: usize) -> Elem {
    ui::swiper::slide(
        &[
            class("flex-1 flex flex-col items-center justify-center"),
            hx::get(
                &route::Route::Feed(Route::LoadMore {
                    feed_id: feed_id.clone(),
                    start_feed_index,
                })
                .encode(),
            ),
            hx::Trigger::Intersect.into(),
            hx::Swap::OuterHtml.into(),
        ],
        &[ui::image::view(&[
            src(" "),
            class("w-full h-full object-cover"),
        ])],
    )
}

fn to_media_details_route(media_id: &MediaId) -> route::Route {
    route::Route::Media(media::route::Route::Details(
        media::details::route::Route::Index {
            media_id: media_id.clone(),
        },
    ))
}

fn view_feed_item(feed_item: &FeedItem) -> Elem {
    ui::swiper::slide(
        &[
            class(
                "w-full h-full flex flex-col items-center justify-center cursor-pointer relative",
            ),
            attr("data-feed-index", &feed_item.to_feed_index().to_string()),
        ],
        &[view_feed_item_content(feed_item)],
    )
}

fn view_feed_item_content(feed_item: &FeedItem) -> Elem {
    match feed_item {
        FeedItem::Media {
            media,
            feed_index: _,
        } => button(
            &[
                class("w-full h-full"),
                hx::get(&to_media_details_route(&media.media_id).encode()),
                hx::Trigger::Click.into(),
                hx::Preload::MouseDown.into(),
                hx::Swap::InnerHtml.into(),
                hx::push_url("true"),
                hx::target(ROOT_SELECTOR),
            ],
            &[ui::image::view(&[
                class("w-full h-full object-cover"),
                width("100%"),
                height("100%"),
                src(media.media_poster.to_highest_res()),
            ])],
        ),
    }
}

fn view_load_initial(model: &ViewModel) -> Elem {
    div(
        &[
            class("flex-1 flex flex-col items-center justify-center"),
            hx::get(
                &route::Route::Feed(Route::LoadInitial {
                    feed_id: model.feed.feed_id.clone(),
                })
                .encode(),
            ),
            hx::Trigger::Load.into(),
            hx::Swap::OuterHtml.into(),
        ],
        &[ui::image::view(&[
            src(" "),
            class("w-full h-full object-cover"),
        ])],
    )
}
