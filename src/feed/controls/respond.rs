use super::{ctx::Ctx, route::Route, view_model::ViewModel};
use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        params::Params,
        ui::{
            self,
            button::{Button, Color},
            chip::ChipSize,
            icon::{self},
            search_bar::SearchBar,
            spinner_page,
        },
    },
    feed::{self, feed_::Feed, feed_id::FeedId},
    req::Req,
    route,
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Index { feed_id } => {
            sse()
                .event_merge_fragments()
                .data_fragments(view_index_loading(&feed_id))
                .send(w)
                .await?;

            let model = ViewModel::load(ctx, feed_id, "").await;

            sse()
                .event_merge_fragments()
                .data_fragments(view_index(&model))
                .send(w)
                .await?;

            let signal_selected_tag_ids = model
                .form_state
                .tags
                .iter()
                .map(|t| format!("'{}'", t.encode()))
                .collect::<Vec<String>>()
                .join(",");

            sse()
                .event_merge_signals()
                .data_signals(&format!(
                    "{{signalSelectedTagIds: [{}]}}",
                    signal_selected_tag_ids
                ))
                .send(w)
                .await?;

            Ok(())
        }

        Route::ClickedSave { feed_id } => {
            let model = ViewModel::load(ctx, feed_id, "").await;

            let view = view_index(&model);

            let feed_new = Feed {
                start_index: 0,
                tags: model.form_state.tags,
                ..model.feed
            };

            ctx.feed_db.put(feed_new.clone()).await.unwrap_or(());

            sse()
                .event_merge_fragments()
                .data_fragments(view)
                .send(w)
                .await
        }

        Route::ClickedTag { feed_id, tag } => {
            let model = ViewModel::load(ctx, feed_id, "").await;

            let form_state_new = model.form_state.toggle(tag);

            ctx.form_state_db.put(&form_state_new).await.unwrap_or(());

            Ok(())
        }

        Route::InputtedSearch { feed_id } => {
            let default = "".to_string();

            let search_input = r.params.get_first("signalInputValue").unwrap_or(&default);

            let model = ViewModel::load(ctx, feed_id, search_input).await;

            sse()
                .event_merge_fragments()
                .data_fragments(view_search_results(&model))
                .send(w)
                .await
        }

        Route::ClickedGoBack { feed_id: _ } => Ok(()),
    }
}

fn to_back_route(feed_id: FeedId) -> route::Route {
    route::Route::Feed(feed::route::Route::Index { feed_id })
}

fn view_root() -> Elem {
    div()
        .id_root()
        .data_store("{signalInputValue: '', signalSelectedTagIds: []}")
        .child(code().child(pre().data_text("JSON.stringify(ctx.store(),null,2)")))
        .class("w-full h-full flex flex-col overflow-hidden relative")
}

fn view_search_input(feed_id: &FeedId, loading_path: &str) -> Elem {
    SearchBar::new()
        .search_url(
            &route::Route::Feed(feed::route::Route::Controls(Route::InputtedSearch {
                feed_id: feed_id.clone(),
            }))
            .encode(),
        )
        .search_results_id(SEARCH_RESULTS_ID)
        .input_id("feed-controls-search-input")
        .input_model("signalInputValue")
        .view()
        .child(view_close_button())
}

fn view_close_button() -> Elem {
    button()
        .type_("button")
        .tab_index(0)
        .aria_label("close")
        .class("h-full pr-5 place-items-center")
        .class("hidden peer-placeholder-shown:grid")
        .child(icon::x_mark("size-6"))
}

fn view_index_loading(feed_id: &FeedId) -> Elem {
    view_root()
        .child(view_search_input(&feed_id, ""))
        .child(spinner_page::view())
        .child(view_section_bottom_bar(&feed_id, ""))
}

fn view_index(model: &ViewModel) -> Elem {
    let clicked_save_path = route::Route::Feed(feed::route::Route::Controls(Route::ClickedSave {
        feed_id: model.feed.feed_id.clone(),
    }))
    .encode();

    view_root()
        .child(view_search_input(&model.feed.feed_id, &clicked_save_path))
        .child(
            div()
                .class("m-0 w-full flex-1 flex flex-col overflow-hidden relative")
                .child(view_search_results(model))
                .child(view_section_bottom_bar(
                    &model.feed.feed_id,
                    &clicked_save_path,
                )),
        )
}

fn view_section_bottom_bar(feed_id: &FeedId, loading_path: &str) -> Elem {
    div()
        .class("flex-none flex flex-row items-center justify-center p-4 border-t gap-4")
        .child(
            Button::new()
                .label("Cancel")
                .color(Color::Gray)
                .loading_disabled_path(&loading_path)
                .view()
                .data_on_click_push_then_get(&to_back_route(feed_id.clone()).encode())
                .type_("button")
                .class("flex-1"),
        )
        .child(
            Button::new()
                .label("Save")
                .color(ui::button::Color::Primary)
                .loading_path(&loading_path)
                .view()
                .class("flex-1"),
        )
}

const SEARCH_RESULTS_ID: &str = "search-results";

fn view_search_results(model: &ViewModel) -> Elem {
    div()
        .id(SEARCH_RESULTS_ID)
        .class("flex-1 flex flex-col p-4 pt-5 overflow-y-auto")
        .child(view_search_results_content(&model))
}

fn view_search_results_content(model: &ViewModel) -> Elem {
    if model.tags.len() == 0 {
        return div()
            .class("w-full overflow-hidden flex-1 flex items-start justify-start font-bold text-lg break-all")
            .child_text(&format!(r#"No results for "{}""#, model.search_input));
    }

    div()
        .data_on(
            "change",
            "let v = evt.target.value; $signalSelectedTagIds = $signalSelectedTagIds.includes(v) ? $signalSelectedTagIds.filter(v_ => v_ !== v) : $signalSelectedTagIds.concat(v)",
        )
        .class("flex flex-row items-start justify-start flex-wrap gap-2")
        .child(view_search_results_frag(&model))
}

fn view_search_results_frag(model: &ViewModel) -> Elem {
    frag().children(
        model
            .to_tags()
            .iter()
            .map(|feed_tag| {
                feed_tag
                    .chip()
                    .size(ChipSize::Large)
                    .bind_checked(&format!(
                        "$signalSelectedTagIds.includes('{}')",
                        feed_tag.encode()
                    ))
                    .view()
            })
            .collect::<Vec<Elem>>(),
    )
}
