use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        query::{Query, QueryFilter, QueryOp},
        ui::{error, image::Image, top_bar::TopBar},
    },
    ctx::Ctx,
    media::{
        interaction::interaction_form::{
            self, interaction_form_view_config::InteractionFormViewConfig,
        },
        media_::Media,
        media_db::interface::MediaQueryField,
        media_id::MediaId,
    },
    req::Req,
};

use super::route::Route;

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    match route {
        Route::MediaDetailsScreen { media_id, back_url } => {
            let model = ViewModel::Loading {
                back_url: back_url.clone(),
                media_id: media_id.clone(),
            };

            w.send_screen(r, model.view_screen()).await?;

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
                    w.send_screen(r, error::screen("Media not found")).await?;
                    return Ok(());
                }
                Some(media) => media,
            };

            let model = ViewModel::Loaded {
                media,
                back_url: back_url.clone(),
                media_id: media_id.clone(),
            };

            w.send_screen(r, model.view_screen()).await?;

            let user_id = r.user_id(ctx).await?;

            interaction_form::respond::respond_interaction_form(
                ctx,
                w,
                user_id,
                vec![to_interaction_form_view_config(media_id.clone())],
            )
            .await?;

            Ok(())
        }
    }
}

fn to_interaction_form_view_config(media_id: MediaId) -> InteractionFormViewConfig {
    InteractionFormViewConfig::default()
        .media_id(media_id)
        .namespace("media-details".to_owned())
        .orientation_horizontal()
}

enum ViewModel {
    Loading {
        back_url: String,
        media_id: MediaId,
    },
    Loaded {
        back_url: String,
        media_id: MediaId,
        media: Media,
    },
}

impl ViewModel {
    fn back_url(&self) -> String {
        match self {
            ViewModel::Loading { back_url, .. } => back_url.clone(),
            ViewModel::Loaded { back_url, .. } => back_url.clone(),
        }
    }

    fn media_id(&self) -> MediaId {
        match self {
            ViewModel::Loading { media_id, .. } => media_id.clone(),
            ViewModel::Loaded { media_id, .. } => media_id.clone(),
        }
    }

    fn view_screen(&self) -> Html {
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

    fn view_top_bar(&self) -> Html {
        let title: &str = match self {
            ViewModel::Loading { .. } => " ",
            ViewModel::Loaded { media, .. } => &media.title,
        };

        TopBar::default()
            .back_url(self.back_url())
            .title(title)
            .view()
            .id("top-bar")
    }

    fn view_backdrop(&self) -> Html {
        let src: &str = match self {
            ViewModel::Loading { .. } => " ",
            ViewModel::Loaded { media, .. } => &media.backdrop.to_highest_res(),
        };

        div()
            .id("backdrop")
            .class("w-full aspect-video overflow-hidden border-b_")
            .child(
                Image::new()
                    .view()
                    .src(src)
                    .class("w-full h-full select-none"),
            )
    }

    fn view_content(&self) -> Html {
        div()
            .id("content")
            .class("flex flex-col gap-4 items-center")
            .child(self.view_content_title())
            .child(to_interaction_form_view_config(self.media_id()).view())
            .child(self.view_content_description())
    }

    fn view_content_title(&self) -> Html {
        match self {
            ViewModel::Loading { .. } => frag(),
            ViewModel::Loaded { media, .. } => div()
                .class("text-3xl font-bold text-center px-6")
                .child_text(&media.title),
        }
    }

    fn view_content_description(&self) -> Html {
        match self {
            ViewModel::Loading { .. } => frag(),
            ViewModel::Loaded { media, .. } => p()
                .class("text-base text-opacity font-normal text-center px-6")
                .child_text(&media.description),
        }
    }
}
