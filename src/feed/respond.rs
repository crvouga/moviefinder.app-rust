use super::{
    feed_::Feed,
    feed_screen::{get_feed_items, respond_screen_contents, view_screen, view_slide, BOTTOM_ID},
    feed_tags,
    route::Route,
};
use crate::{
    core::{
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        unstructured_data::UnstructuredData,
    },
    ctx::Ctx,
    feed::feed_screen::transact_put_feed,
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
            let maybe_slide_index = r
                .params
                .get_first("signalFeedIndex")
                .and_then(|s| s.parse::<usize>().ok());

            let slide_index_new = match maybe_slide_index {
                None => return Ok(()),

                Some(slide_index_new) => slide_index_new,
            };

            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let feed_new = Feed {
                start_index: slide_index_new,
                ..feed
            };

            transact_put_feed(ctx, &r.session_id, &feed_new).await?;

            Ok(())
        }

        Route::IntersectedBottom {
            feed_id,
            bottom_feed_index,
        } => {
            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let feed_with_new_index = Feed {
                start_index: *bottom_feed_index + 1,
                ..feed
            };

            let feed_items = get_feed_items(ctx, &feed_with_new_index)
                .await
                .unwrap_or_default();

            for feed_item in feed_items {
                sse()
                    .event_merge_fragments()
                    .data_selector_id(BOTTOM_ID)
                    .data_merge_mode_before()
                    .data_fragments(view_slide(&feed_item))
                    .send(w)
                    .await?;
            }

            Ok(())
        }

        Route::Tags(child) => feed_tags::respond::respond(&ctx, r, child, w).await,
    }
}
