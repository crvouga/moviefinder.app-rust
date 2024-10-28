use crate::{
    core::{
        html::*,
        query::{Filter, Op, Query},
        res::Res,
        ui::{self, image::Image},
    },
    ctx::Ctx,
    feed,
    media::{self, core::Media, media_db::interface::MediaField, media_id::MediaId},
    route,
    ui::{root::ROOT_SELECTOR, top_bar::TopBar},
};

use super::route::Route;

pub async fn respond(ctx: &Ctx, route: &Route) -> Res {
    match route {
        Route::Index { media_id } => view_load(media_id).into(),

        Route::Load { media_id } => {
            // tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let query = Query {
                limit: 1,
                offset: 0,
                filter: Filter::clause(MediaField::MediaId, Op::Eq, media_id.as_str().to_string()),
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

#[derive(Default)]
struct Layout {
    children: Vec<Elem>,
    media: Option<Media>,
}

impl Layout {
    pub fn new() -> Self {
        Layout::default()
    }

    pub fn child(mut self, child: Elem) -> Self {
        self.children.push(child);
        self
    }

    pub fn media(mut self, media: Media) -> Self {
        self.media = Some(media);
        self
    }

    pub fn view(self) -> Elem {
        let top_bar_title = self.media.as_ref().map_or("", |m| &m.media_title);

        let image_src = self
            .media
            .as_ref()
            .map_or(" ", |m| m.media_backdrop.to_highest_res());

        div()
            .class("flex flex-col")
            .child(
                TopBar::new()
                    .back_button(route::Route::Feed(feed::route::Route::Index))
                    .title(top_bar_title)
                    .view(),
            )
            .child(
                div().class("flex flex-col gap-6 items-center").child(
                    div()
                        .class("w-full aspect-video overflow-hidden border-b")
                        .child(
                            Image::view()
                                .class("w-full h-full select-none")
                                .src(image_src),
                        )
                        .children(&self.children),
                ),
            )
    }
}

fn view_load(media_id: &MediaId) -> Elem {
    Layout::new()
        .child(ui::icon::spinner("animate-spin size-16"))
        .view()
        .hx_trigger_load()
        .hx_swap_inner_html()
        .hx_target(ROOT_SELECTOR)
        .hx_get(&load_route(media_id).encode())
}

fn view_details(media: &Media) -> Elem {
    Layout::new()
        .media(media.clone())
        .child(
            div()
                .class("flex flex-col gap-4 items-center")
                .child(view_title(media))
                .child(view_description(media)),
        )
        .view()
}

fn view_title(media: &Media) -> Elem {
    p().class("text-3xl font-bold text-center px-6")
        .child_text(&media.media_title)
}

fn view_description(media: &Media) -> Elem {
    p().class("text-base text-opacity font-normal text-center px-6")
        .child_text(&media.media_description)
}

fn load_route(media_id: &MediaId) -> route::Route {
    route::Route::Media(media::route::Route::Details(
        media::details::route::Route::Load {
            media_id: media_id.clone(),
        },
    ))
}
