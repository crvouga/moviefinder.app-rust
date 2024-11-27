use super::{ctx::Ctx, route::Route, view_model::ViewModel};
use crate::{
    core::{
        html::*,
        http::{request::Request, response_writer::ResponseWriter, server_sent_event::sse},
        params::Params,
        ui::{
            self,
            button::{Button, Color},
            chip::ChipSize,
            icon::{self},
            spinner_page,
        },
    },
    feed::{self, feed_::Feed, feed_id::FeedId},
    route,
};

const FEED_TAG_ID_NAME: &str = "feed_tag_id";
const SEARCH_NAME: &str = "search";

const TAGS_ID: &str = "feed-tags";
fn tags_selector() -> String {
    format!("#{}", TAGS_ID)
}

const INDEX_ID: &str = "feed-controls";
fn index_selector() -> String {
    format!("#{}", INDEX_ID)
}

pub async fn respond(
    ctx: &Ctx,
    r: &Request,
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

            Ok(())
        }

        Route::ClickedSave { feed_id } => {
            let model = ViewModel::load(ctx, feed_id, "").await;

            let feed_new = Feed {
                start_index: 0,
                tags: model.form_state.tags,
                ..model.feed
            };

            ctx.feed_db.put(feed_new.clone()).await.unwrap_or(());

            Ok(())
        }

        Route::ClickedTag { feed_id, tag } => {
            let model: ViewModel = ViewModel::load(ctx, feed_id, "").await;

            let form_state_new = model.form_state.toggle(tag);

            ctx.form_state_db.put(&form_state_new).await.unwrap_or(());

            Ok(())
        }

        Route::InputtedSearch { feed_id } => {
            let default = "".to_string();

            let params = r.params();

            let search_input = params.get_first(SEARCH_NAME).unwrap_or(&default);

            let model = ViewModel::load(ctx, feed_id, search_input).await;

            let _ = view_section_tags(&model);

            Ok(())
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
        .class("w-full h-full flex flex-col overflow-hidden relative")
}

fn view_section_search_bar(feed_id: &FeedId, loading_path: &str) -> Elem {
    ui::search_bar::SearchBar::new()
        .inputted_search_path(
            &route::Route::Feed(feed::route::Route::Controls(Route::InputtedSearch {
                feed_id: feed_id.clone(),
            }))
            .encode(),
        )
        .inputted_search_target(&tags_selector())
        .input_search_id("feed-controls-search-input")
        .input_search_name(SEARCH_NAME)
        .view()
        .child(
            button()
                .type_("button")
                .tab_index(0)
                .aria_label("close")
                .hx_loading_disabled()
                .hx_loading_path(loading_path)
                .hx_abort(&index_selector())
                .hx_push_root_route(to_back_route(feed_id.clone()))
                .class("h-full pr-5 place-items-center")
                .class("hidden peer-placeholder-shown:grid")
                .child(icon::x_mark("size-6")),
        )
}

fn view_index_loading(feed_id: &FeedId) -> Elem {
    view_root()
        .child(view_section_search_bar(&feed_id, ""))
        .child(
            spinner_page::view()
                .hx_swap_root_route(route::Route::Feed(feed::route::Route::Controls(
                    Route::Index {
                        feed_id: feed_id.clone(),
                    },
                )))
                .hx_trigger_load()
                .id(INDEX_ID),
        )
        .child(view_section_bottom_bar(&feed_id, ""))
}

fn view_index(model: &ViewModel) -> Elem {
    let clicked_save_path = route::Route::Feed(feed::route::Route::Controls(Route::ClickedSave {
        feed_id: model.feed.feed_id.clone(),
    }))
    .encode();

    view_root()
        .child(view_section_search_bar(
            &model.feed.feed_id,
            &clicked_save_path,
        ))
        .child(
            form()
                .class("m-0 w-full flex-1 flex flex-col overflow-hidden relative")
                .hx_post(&clicked_save_path)
                .hx_swap_none()
                .child(view_section_tags(model))
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
                .class("flex-1")
                .hx_abort(&index_selector()),
        )
        .child(
            Button::new()
                .label("Save")
                .color(ui::button::Color::Primary)
                .loading_path(&loading_path)
                .view()
                .type_("submit")
                .class("flex-1"),
        )
}

fn view_section_tags(model: &ViewModel) -> Elem {
    div()
        .id(TAGS_ID)
        .class("flex-1 flex flex-col p-4 pt-5 overflow-y-auto")
        .child(view_tag_chips(&model))
}

fn view_tag_chips(model: &ViewModel) -> Elem {
    if model.tags.len() == 0 {
        return div()
            .class("w-full overflow-hidden flex-1 flex items-start justify-start font-bold text-lg break-all")
            .child_text(&format!(r#"No results for "{}""#, model.search_input));
    }

    div()
        .class("flex flex-row items-start justify-start flex-wrap gap-2")
        .child(view_tag_chips_frag(&model))
}

fn view_tag_chips_frag(model: &ViewModel) -> Elem {
    frag().children(
        model
            .to_tags()
            .iter()
            .map(|feed_tag| {
                feed_tag
                    .chip()
                    .size(ChipSize::Large)
                    .checked(model.form_state.tags.contains(feed_tag))
                    .disabled(false)
                    .name(FEED_TAG_ID_NAME)
                    .view()
                    .hx_trigger_click()
                    .hx_swap_none()
                    .hx_include_this()
                    .hx_post(
                        &route::Route::Feed(feed::route::Route::Controls(Route::ClickedTag {
                            feed_id: model.feed.feed_id.clone(),
                            tag: feed_tag.clone(),
                        }))
                        .encode(),
                    )
            })
            .collect::<Vec<Elem>>(),
    )
}
