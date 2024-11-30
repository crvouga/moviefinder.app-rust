use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        query::{Query, QueryFilter, QueryOp},
        ui::image::Image,
    },
    ctx::Ctx,
    feed,
    media::{media_::Media, media_db::interface::MediaQueryField, media_id::MediaId},
    req::Req,
    route,
    ui::top_bar::TopBar,
};

use super::route::Route;

fn to_screen_id(media_id: &MediaId, child_id: &str) -> String {
    let media_id = media_id.as_str();
    let child_id = child_id.trim();
    let prefix = "media";

    if media_id.is_empty() && child_id.is_empty() {
        prefix.to_string()
    } else if media_id.is_empty() {
        format!("{}-{}", prefix, child_id)
    } else if child_id.is_empty() {
        format!("{}-{}", prefix, media_id)
    } else {
        format!("{}-{}-{}", prefix, media_id, child_id)
    }
}

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen { media_id } => {
            sse()
                .send_screen(r, w, &to_screen_id(media_id, ""), view_screen(media_id))
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
                        .data_fragments(view_backdrop(&media_id, &media))
                        .send(w)
                        .await?;

                    sse()
                        .event_merge_fragments()
                        .data_merge_mode_outer()
                        .data_fragments(view_top_bar(&media_id, &media))
                        .send(w)
                        .await?;

                    sse()
                        .event_merge_fragments()
                        .data_merge_mode_outer()
                        .data_fragments(view_content(&media_id, &media))
                        .send(w)
                        .await?;
                    Ok(())
                }
                None => Ok(()),
            }
        }
    }
}

fn view_screen(media_id: &MediaId) -> Elem {
    div()
        .id(&to_screen_id(media_id, ""))
        .class("flex flex-col")
        .child(view_top_bar_loading(media_id))
        .child(
            div()
                .class("flex flex-col gap-6 items-center")
                .child(view_backdrop_loading(media_id))
                .child(view_content_loading(media_id)),
        )
}

fn view_top_bar_root(media_id: &MediaId, title: &str) -> Elem {
    TopBar::default()
        .back_button(route::Route::Feed(feed::route::Route::ScreenDefault))
        .title(title)
        .view()
        .id(&to_screen_id(media_id, "top-bar"))
}

fn view_top_bar_loading(media_id: &MediaId) -> Elem {
    view_top_bar_root(media_id, " ")
}

fn view_top_bar(media_id: &MediaId, media: &Media) -> Elem {
    view_top_bar_root(media_id, &media.title)
}

fn view_backdrop_root(media_id: &MediaId, src: &str) -> Elem {
    div()
        .id(&to_screen_id(media_id, "backdrop"))
        .class("w-full aspect-video overflow-hidden border-b")
        .child(
            Image::new()
                .view()
                .src(&src)
                .class("w-full h-full select-none"),
        )
}

fn view_backdrop_loading(media_id: &MediaId) -> Elem {
    view_backdrop_root(media_id, " ")
}

fn view_backdrop(media_id: &MediaId, media: &Media) -> Elem {
    view_backdrop_root(media_id, &media.backdrop.to_highest_res())
}

fn view_content_root(media_id: &MediaId) -> Elem {
    div()
        .id(&to_screen_id(media_id, "content"))
        .class("flex flex-col gap-4 items-center")
}

fn view_content_loading(media_id: &MediaId) -> Elem {
    view_content_root(media_id)
}

fn view_content(media_id: &MediaId, media: &Media) -> Elem {
    view_content_root(media_id)
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
