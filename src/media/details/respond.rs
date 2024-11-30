use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        query::{Query, QueryFilter, QueryOp},
        ui::image::Image,
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
            // sse()
            //     .event_merge_fragments()
            //     .data_fragments(view_screen(&r.path))
            //     .send(w)
            //     .await?;

            sse().send_screen(r, w, view_screen(&r.path)).await?;

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
                        .data_fragments(view_backdrop(&r.path, &media))
                        .send(w)
                        .await?;

                    sse()
                        .event_merge_fragments()
                        .data_merge_mode_outer()
                        .data_fragments(view_top_bar(&r.path, &media))
                        .send(w)
                        .await?;

                    sse()
                        .event_merge_fragments()
                        .data_merge_mode_outer()
                        .data_fragments(view_content(&r.path, &media))
                        .send(w)
                        .await?;
                    Ok(())
                }
                None => Ok(()),
            }
        }
    }
}

fn view_screen(path: &str) -> Elem {
    div()
        .id(path)
        .namespace_children_ids(path)
        .class("flex flex-col")
        .child(view_top_bar_loading(path))
        .child(
            div()
                .class("flex flex-col gap-6 items-center")
                .child(view_backdrop_loading(path))
                .child(view_content_loading(path)),
        )
}

fn view_top_bar_root(path: &str, title: &str) -> Elem {
    TopBar::default()
        .back_button(route::Route::Feed(feed::route::Route::ScreenDefault))
        .title(title)
        .view()
        .id(&format!("{}-top-bar", path))
}

fn view_top_bar_loading(path: &str) -> Elem {
    view_top_bar_root(path, " ")
}

fn view_top_bar(path: &str, media: &Media) -> Elem {
    view_top_bar_root(path, &media.title)
}

fn view_backdrop_root(path: &str, src: &str) -> Elem {
    div()
        .id(&format!("{}-backdrop", path))
        .class("w-full aspect-video overflow-hidden border-b")
        .child(
            Image::new()
                .view()
                .src(&src)
                .class("w-full h-full select-none"),
        )
}

fn view_backdrop_loading(path: &str) -> Elem {
    view_backdrop_root(path, " ")
}

fn view_backdrop(path: &str, media: &Media) -> Elem {
    view_backdrop_root(path, &media.backdrop.to_highest_res())
}

fn view_content_root(path: &str) -> Elem {
    div()
        .id(&format!("{}-content", path))
        .class("flex flex-col gap-4 items-center")
}

fn view_content_loading(path: &str) -> Elem {
    view_content_root(path)
}

fn view_content(path: &str, media: &Media) -> Elem {
    view_content_root(path)
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
