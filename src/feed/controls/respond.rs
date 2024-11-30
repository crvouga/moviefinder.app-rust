use super::{ctx::Ctx, form_state::FormState, route::Route, view_model::ViewModel};
use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        params::Params,
        ui::{
            self,
            button::{Button, Color},
            chip::ChipSize,
            search_bar::SearchBar,
            spinner_page,
        },
    },
    feed::{self, feed_::Feed, feed_id::FeedId, feed_tag::FeedTag},
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
                .map(|t| format!("'{}'", t.encode().to_lowercase()))
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

            let feed_new = Feed {
                start_index: 0,
                tags: model.form_state.tags,
                ..model.feed
            };

            ctx.feed_db.put(feed_new.clone()).await.unwrap_or(());

            let back = &to_back_route(feed_id.clone()).encode();

            sse()
                .event_execute_script()
                .data_script_redirect(back)
                .send(w)
                .await
        }

        Route::ClickedTag { feed_id } => {
            let selected_tags_new = r.to_selected_tags();

            let signal_input_value = r.params.get_first("signalInputValue");

            let model =
                ViewModel::load(ctx, feed_id, signal_input_value.unwrap_or(&"".to_string())).await;

            let form_state_new = FormState {
                feed_id: model.feed.feed_id.clone(),
                tags: selected_tags_new,
            };

            let model_new = ViewModel {
                form_state: form_state_new.clone(),
                ..model
            };

            sse()
                .event_merge_fragments()
                .data_fragments(view_selected(&model_new))
                .send(w)
                .await?;

            sse()
                .event_merge_fragments()
                .data_fragments(view_unselected(&model_new))
                .send(w)
                .await?;

            ctx.form_state_db.put(&form_state_new).await.unwrap_or(());

            Ok(())
        }

        Route::InputtedSearch { feed_id } => {
            let default = "".to_string();

            let search_input = r.params.get_first("signalInputValue").unwrap_or(&default);

            let model = ViewModel::load(ctx, feed_id, search_input).await;

            let selected_tags_new = r.to_selected_tags();
            let model_new = ViewModel {
                form_state: FormState {
                    tags: selected_tags_new,
                    ..model.form_state
                },
                ..model
            };

            sse()
                .event_merge_fragments()
                .data_fragments(view_unselected(&model_new))
                .send(w)
                .await?;

            sse()
                .event_merge_fragments()
                .data_fragments(view_selected(&model_new))
                .send(w)
                .await
        }

        Route::ClickedGoBack { feed_id: _ } => Ok(()),
    }
}

impl Req {
    pub fn to_selected_tags(&self) -> Vec<FeedTag> {
        let signal_selected_tag_ids = self.params.get_all("signalSelectedTagIds");

        let selected_tags = signal_selected_tag_ids
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|tag| FeedTag::decode(tag))
            .collect::<Vec<FeedTag>>();

        selected_tags
    }
}

fn to_back_route(feed_id: FeedId) -> route::Route {
    route::Route::Feed(feed::route::Route::Index { feed_id })
}

fn view_root(feed_id: &FeedId) -> Elem {
    div()
        .id_root()
        .data_store("{signalInputValue: '', signalSelectedTagIds: []}")
        .data_on_store_change("window.ctx = ctx")
        // .child_debug_store()
        .data_toggle_clicked_tag()
        .data_on_patch(
            "clicked-tag",
            &route::Route::Feed(feed::route::Route::Controls(Route::ClickedTag {
                feed_id: feed_id.clone(),
            }))
            .encode(),
        )
        .class("w-full h-full flex flex-col overflow-hidden relative")
}

fn view_search_input(feed_id: &FeedId) -> Elem {
    SearchBar::new()
        .search_url(
            &route::Route::Feed(feed::route::Route::Controls(Route::InputtedSearch {
                feed_id: feed_id.clone(),
            }))
            .encode(),
        )
        .input_id("search-input")
        .input_model("signalInputValue")
        .view()
}

fn view_selected_root() -> Elem {
    div().id("selected-tags").class(
        "flex-none flex flex-row items-center justify-start p-2 gap-2 min-h-16 flex-wrap border-b",
    )
}

fn js_signal_is_checked(tag: &FeedTag) -> String {
    format!(
        "$signalSelectedTagIds.includes('{}')",
        tag.encode().to_lowercase()
    )
}

impl Elem {
    fn data_toggle_clicked_tag(self) -> Self {
        self.data_on(
            "clicked-tag",
            "let v = evt.target.id.toLowerCase(); $signalSelectedTagIds = $signalSelectedTagIds.includes(v.toLowerCase()) ? $signalSelectedTagIds.filter(v_ => v_.toLowerCase() !== v.toLowerCase()) : [...$signalSelectedTagIds, v.toLowerCase()]",
        )
    }
    fn data_on_clicked_tag(self) -> Self {
        self.data_on_click(
            "evt.target.dispatchEvent(new CustomEvent('clicked-tag', { bubbles: true }))",
        )
    }
}

fn view_selected(model: &ViewModel) -> Elem {
    view_selected_root().children(
        model
            .to_all_tags()
            .iter()
            .map(|tag| {
                tag.chip()
                    .size(ChipSize::Small)
                    .id(&tag.encode().to_lowercase())
                    .signal_checked(&js_signal_is_checked(tag))
                    .view()
                    .data_show(&js_signal_is_checked(tag))
                    .data_on_clicked_tag()
            })
            .collect::<Vec<Elem>>(),
    )
}

fn view_index_loading(feed_id: &FeedId) -> Elem {
    view_root(&feed_id)
        .child(view_selected_root())
        .child(view_search_input(&feed_id))
        .child(spinner_page::view())
        .child(view_bottom_bar(&feed_id))
}

fn view_index(model: &ViewModel) -> Elem {
    view_root(&model.feed.feed_id)
        .child(view_selected(&model))
        .child(view_search_input(&model.feed.feed_id))
        .child(view_unselected(model))
        .child(view_bottom_bar(&model.feed.feed_id))
}

fn view_bottom_bar(feed_id: &FeedId) -> Elem {
    div()
        .class("flex-none flex flex-row items-center justify-center p-4 border-t gap-4")
        .child(
            Button::new()
                .label("Cancel")
                .color(Color::Gray)
                .view()
                .data_on_click_push_then_get(&to_back_route(feed_id.clone()).encode())
                .type_("button")
                .class("flex-1"),
        )
        .child(
            Button::new()
                .label("Save")
                .color(ui::button::Color::Primary)
                .indicator("isSaving")
                .view()
                .data_on_click_post(
                    &route::Route::Feed(feed::route::Route::Controls(Route::ClickedSave {
                        feed_id: feed_id.clone(),
                    }))
                    .encode(),
                )
                .id("save-button")
                .class("flex-1"),
        )
}

fn view_unselected(model: &ViewModel) -> Elem {
    div()
        .id("unselected-tags")
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
                    .id(&feed_tag.encode())
                    .signal_checked(&js_signal_is_checked(feed_tag))
                    .view()
                    .data_on_clicked_tag()
            })
            .collect::<Vec<Elem>>(),
    )
}
