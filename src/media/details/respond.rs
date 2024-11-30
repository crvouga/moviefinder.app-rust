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
    _r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen { media_id } => {
            sse()
                .event_merge_fragments()
                .data_fragments(view_index_loading())
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

            let result = match queried {
                Ok(result) => result,
                Err(err) => {
                    sse()
                        .event_merge_fragments()
                        .data_fragments(ui::error::page(&err).id_root())
                        .send(w)
                        .await?;
                    return Ok(());
                }
            };

            let media = match result.items.into_iter().next() {
                Some(media) => media,
                None => {
                    sse()
                        .event_merge_fragments()
                        .data_fragments(ui::error::page("Media not found").id_root())
                        .send(w)
                        .await?;
                    return Ok(());
                }
            };

            sse()
                .event_merge_fragments()
                .data_merge_mode_outer()
                .data_fragments(view_index(&media))
                .send(w)
                .await?;

            Ok(())
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
            .id_root()
            .class("flex flex-col")
            .child(
                TopBar::default()
                    .back_button(route::Route::Feed(feed::route::Route::DefaultScreen))
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

fn view_index_loading() -> Elem {
    Layout::new()
        // .child(ui::icon::spinner("animate-spin size-16"))
        .view()
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
        .child_text(&media.title)
}

fn view_description(media: &Media) -> Elem {
    p().class("text-base text-opacity font-normal text-center px-6")
        .child_text(&media.description)
}
