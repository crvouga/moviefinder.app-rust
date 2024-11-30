use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        query::{Query, QueryFilter, QueryOp},
        ui::{self, image::Image},
    },
    ctx::Ctx,
    feed,
    media::{media_::Media, media_db::interface::MediaQueryField},
    req::Req,
    route,
    ui::top_bar::TopBar,
};

use super::route::Route;

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen { media_id } => {
            sse()
                .event_merge_fragments()
                .data_fragments(view_screen(&r.path))
                .send(w)
                .await?;

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

            let media = queried.unwrap_or_default().items.into_iter().next();

            match media {
                Some(media) => {
                    sse()
                        .event_merge_fragments()
                        .data_merge_mode_outer()
                        .data_fragments(view_backdrop(&media))
                        .send(w)
                        .await?;

                    sse()
                        .event_merge_fragments()
                        .data_merge_mode_outer()
                        .data_fragments(view_top_bar(&media))
                        .send(w)
                        .await?;

                    sse()
                        .event_merge_fragments()
                        .data_merge_mode_outer()
                        .data_fragments(view_content(&media))
                        .send(w)
                        .await?;
                    Ok(())
                }
                None => Ok(()),
            }
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
        let top_bar_title = self.media.as_ref().map_or("", |m| &m.title);

        let image_src = self
            .media
            .as_ref()
            .map_or(" ", |m| m.backdrop.to_highest_res());

        div()
            .class("flex flex-col")
            .child(
                TopBar::default()
                    .back_button(route::Route::Feed(feed::route::Route::ScreenDefault))
                    .title(top_bar_title)
                    .view(),
            )
            .child(
                div()
                    .class("flex flex-col gap-6 items-center")
                    .child(
                        div()
                            .class("w-full aspect-video overflow-hidden border-b")
                            .child(
                                Image::new()
                                    .view()
                                    .src(&image_src)
                                    .class("w-full h-full select-none"),
                            ),
                    )
                    .children(self.children),
            )
    }
}

fn view_screen(path: &str) -> Elem {
    div()
        .id(path)
        .namespace_children_ids(path)
        .class("flex flex-col")
        .child(view_top_bar_loading())
        .child(
            div()
                .class("flex flex-col gap-6 items-center")
                .child(view_backdrop_loading())
                .child(view_content_loading()),
        )
}

fn view_top_bar_root(title: &str) -> Elem {
    TopBar::default()
        .back_button(route::Route::Feed(feed::route::Route::ScreenDefault))
        .title(title)
        .view()
        .id("top-bar")
}

fn view_top_bar_loading() -> Elem {
    view_top_bar_root(" ")
}

fn view_top_bar(media: &Media) -> Elem {
    view_top_bar_root(&media.title)
}

fn view_backdrop_root(src: &str) -> Elem {
    div()
        .id("backdrop")
        .class("w-full aspect-video overflow-hidden border-b")
        .child(
            Image::new()
                .view()
                .src(&src)
                .class("w-full h-full select-none"),
        )
}

fn view_backdrop_loading() -> Elem {
    view_backdrop_root(" ")
}

fn view_backdrop(media: &Media) -> Elem {
    view_backdrop_root(&media.backdrop.to_highest_res())
}

fn view_content_root() -> Elem {
    div()
        .id("content")
        .class("flex flex-col gap-4 items-center")
}

fn view_content_loading() -> Elem {
    view_content_root()
}

fn view_content(media: &Media) -> Elem {
    div()
        .id("content")
        .class("flex flex-col gap-4 items-center")
        .child(view_title(media))
        .child(view_description(media))
}

fn view_title(media: &Media) -> Elem {
    p().class("text-3xl font-bold text-center px-6")
        .child_text(&media.title)
}

fn view_description(media: &Media) -> Elem {
    p().class("text-base text-opacity font-normal text-center px-6")
        .child_text(&media.description)
}
