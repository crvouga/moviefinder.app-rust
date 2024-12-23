use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        query::{Query, QueryFilter, QueryOp},
        ui::{error, image::Image, top_bar::TopBar},
    },
    ctx::Ctx,
    feed::feed_screen,
    media::{media_::Media, media_db::interface::MediaQueryField},
    req::Req,
    ui::route::AppRoute,
};

use super::route::Route;

pub async fn respond(
    ctx: &Ctx,
    _r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    match route {
        Route::MediaDetailsScreen { media_id } => {
            let model = ViewModel::Loading;

            w.send_screen(model.view_screen()).await?;

            let queried = ctx
                .media_db
                .query(Query {
                    limit: 1,
                    offset: 0,
                    filter: QueryFilter::Clause(
                        MediaQueryField::MediaId,
                        QueryOp::Eq,
                        media_id.as_str().to_string(),
                    ),
                })
                .await
                .unwrap_or_default()
                .items
                .into_iter()
                .next();

            let media = match queried {
                None => {
                    w.send_screen(error::screen("Media not found")).await?;
                    return Ok(());
                }
                Some(media) => media,
            };

            let model = ViewModel::Loaded { media };

            w.send_screen(model.view_screen()).await?;

            Ok(())
        }
    }
}

enum ViewModel {
    Loading,
    Loaded { media: Media },
}

impl ViewModel {
    fn view_screen(&self) -> Elem {
        div()
            .class("flex flex-col")
            .child(self.view_top_bar())
            .child(
                div()
                    .class("flex flex-col gap-6 items-center")
                    .child(self.view_backdrop())
                    .child(self.view_content()),
            )
    }

    fn view_top_bar(&self) -> Elem {
        let title: &str = match self {
            ViewModel::Loading { .. } => " ",
            ViewModel::Loaded { media, .. } => &media.title,
        };

        TopBar::default()
            .back_url(feed_screen::route::Route::FeedScreenDefault.url())
            .title(title)
            .view()
            .id("top-bar")
    }

    fn view_backdrop(&self) -> Elem {
        let src: &str = match self {
            ViewModel::Loading { .. } => " ",
            ViewModel::Loaded { media, .. } => &media.backdrop.to_highest_res(),
        };

        div()
            .id("backdrop")
            .class("w-full aspect-video overflow-hidden border-b")
            .child(
                Image::new()
                    .view()
                    .src(src)
                    .class("w-full h-full select-none"),
            )
    }

    fn view_content(&self) -> Elem {
        div()
            .id("content")
            .class("flex flex-col gap-4 items-center")
            .child(self.view_content_title())
            .child(self.view_content_description())
    }

    fn view_content_title(&self) -> Elem {
        match self {
            ViewModel::Loading { .. } => frag(),
            ViewModel::Loaded { media, .. } => div()
                .class("text-3xl font-bold text-center px-6")
                .child_text(&media.title),
        }
    }

    fn view_content_description(&self) -> Elem {
        match self {
            ViewModel::Loading { .. } => frag(),
            ViewModel::Loaded { media, .. } => p()
                .class("text-base text-opacity font-normal text-center px-6")
                .child_text(&media.description),
        }
    }
}
