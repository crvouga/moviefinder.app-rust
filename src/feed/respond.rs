use super::{
    feed_::Feed,
    feed_screen::{get_feed_items, respond_screen_contents, view_screen, view_slide, BOTTOM_ID},
    feed_tags_form,
    route::Route,
};
use crate::{
    core::{
        dynamic_data::DynamicData,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
    },
    ctx::Ctx,
    feed::feed_screen::put_feed,
    media::interaction::interaction_form,
    req::Req,
    ui::route::Routable,
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::FeedScreenDefault => {
            w.send_screen(view_screen()).await?;

            let maybe_feed_id = ctx
                .feed_session_mapping_db
                .get(r.session_id.clone())
                .await
                .unwrap_or(None);

            let feed_id = maybe_feed_id.unwrap_or_default();

            let feed_url = (Route::FeedScreen {
                feed_id: feed_id.clone(),
            })
            .url();

            w.send_replace_url(&feed_url).await?;

            w.send_screen(view_screen()).await?;

            respond_screen_contents(ctx, r, w, &feed_id).await
        }

        Route::FeedScreen { feed_id } => {
            w.send_screen(view_screen()).await?;

            respond_screen_contents(ctx, r, w, feed_id).await
        }

        Route::ChangedSlide { feed_id } => {
            let signal_feed_index = r
                .payload
                .get_first("signal_feed_index")
                .and_then(|s| s.parse::<usize>().ok());

            let signal_feed_index = match signal_feed_index {
                None => return Ok(()),

                Some(signal_feed_index) => signal_feed_index,
            };

            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let feed_new = Feed {
                start_index: signal_feed_index,
                ..feed
            };

            put_feed(ctx, &r.session_id, &feed_new).await?;

            Ok(())
        }

        Route::IntersectedBottom { feed_id, .. } => {
            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let signal_feed_index = r
                .payload
                .get_first("signal_feed_index")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or_default();

            let feed_with_new_index: Feed = Feed {
                start_index: signal_feed_index + 1,
                ..feed
            };

            let feed_items = get_feed_items(ctx, &feed_with_new_index)
                .await
                .unwrap_or_default();

            let user_id = r.user_id_result(ctx).await?;

            for feed_item in feed_items {
                sse()
                    .event_merge_fragments()
                    .data_selector_id(BOTTOM_ID)
                    .data_merge_mode_before()
                    .data_fragments(view_slide(&feed_item))
                    .send(w)
                    .await?;

                let media_id = feed_item.to_media_id();

                if let Some(media_id) = media_id {
                    interaction_form::respond::respond_interaction_form(
                        ctx,
                        w,
                        user_id.clone(),
                        vec![media_id],
                    )
                    .await?;
                }
            }

            Ok(())
        }

        Route::Tags(child) => feed_tags_form::respond::respond(&ctx, r, child, w).await,
    }
}
