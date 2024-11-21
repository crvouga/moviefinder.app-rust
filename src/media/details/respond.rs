use crate::{
    core::{
        html::*,
        query::{Query, QueryFilter, QueryOp},
        res::Res,
        ui::{self, image::Image},
    },
    ctx::Ctx,
    feed,
    media::{self, media_::Media, media_db::interface::MediaQueryField, media_id::MediaId},
    route,
    ui::top_bar::TopBar,
};

use super::route::Route;

pub async fn respond(ctx: &Ctx, route: &Route) -> Res {
    match route {
        Route::IndexLoad { media_id } => {
            let res: Res = view_index_load(media_id).into();
            res.cache()
        }

        Route::Index { media_id } => {
            let query = Query {
                limit: 1,
                offset: 0,
                filter: QueryFilter::Clause(
                    MediaQueryField::MediaId,
                    QueryOp::Eq,
                    media_id.as_str().to_string(),
                ),
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

            view_index(&media).into()
        }
    }
}

const INDEX_ID: &str = "media-details";
fn index_selector() -> String {
    format!("#{}", INDEX_ID)
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
            .id(INDEX_ID)
            .child(
                TopBar::default()
                    .back_button(route::Route::Feed(feed::route::Route::DefaultLoad))
                    .title(top_bar_title)
                    .view()
                    .hx_abort(&index_selector()),
            )
            .child(
                div()
                    .class("flex flex-col gap-6 items-center")
                    .child(
                        div()
                            .class("w-full aspect-video overflow-hidden border-b")
                            .child(
                                Image::view()
                                    .class("w-full h-full select-none")
                                    .src(image_src),
                            ),
                    )
                    .children(self.children),
            )
    }
}

fn view_index_load(media_id: &MediaId) -> Elem {
    Layout::new()
        .child(ui::icon::spinner("animate-spin size-16"))
        .view()
        .root_swap_route(route::Route::Media(media::route::Route::Details(
            media::details::route::Route::Index {
                media_id: media_id.clone(),
            },
        )))
        .hx_trigger_load()
}

fn view_index(media: &Media) -> Elem {
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
