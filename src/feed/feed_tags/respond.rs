use super::{form_state::FormState, route::Route, view_model::ViewModel};
use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        params::Params,
        ui::{
            self,
            button::{Button, Color},
            chip::ChipSize,
            icon::spinner,
            search_bar::SearchBar,
            spinner_page,
        },
    },
    ctx::Ctx,
    feed::{self, feed_::Feed, feed_id::FeedId, feed_tag::FeedTag, shared::respond_screen},
    req::Req,
    route,
};

pub fn to_screen_id(_feed_id: &FeedId, child_id: &str) -> String {
    return child_id.to_string();
    // let feed_id = feed_id.as_str().trim();
    // let child_id = child_id.trim();
    // let prefix = "feed-tags";

    // if feed_id.is_empty() && child_id.is_empty() {
    //     prefix.to_string()
    // } else if feed_id.is_empty() {
    //     format!("{}-{}", prefix, child_id)
    // } else if child_id.is_empty() {
    //     format!("{}-{}", prefix, feed_id)
    // } else {
    //     format!("{}-{}-{}", prefix, feed_id, child_id)
    // }
}
pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen { feed_id } => {
            sse()
                .send_screen(r, w, &to_screen_id(feed_id, ""), view_screen(&feed_id))
                .await?;

            let model = ViewModel::load(ctx, feed_id, "").await;

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

            sse()
                .event_merge_fragments()
                .data_fragments(view_selected(&model))
                .send(w)
                .await?;

            sse()
                .event_merge_fragments()
                .data_fragments(view_unselected(&model))
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

            sse()
                .event_merge_signals()
                .data_signals("{signalIsSaving: false}")
                .send(w)
                .await?;

            sse()
                .event_execute_script()
                .data_script_push_url(
                    &route::Route::Feed(feed::route::Route::Screen {
                        feed_id: feed_id.clone(),
                    })
                    .encode(),
                )
                .send(w)
                .await?;

            respond_screen(ctx, r, w, feed_id).await?;

            Ok(())
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

            ctx.feed_tags_form_state_db
                .put(&form_state_new)
                .await
                .unwrap_or(());

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
                .await?;

            Ok(())
        }

        Route::ClickedClear { feed_id } => {
            let model = ViewModel::load(ctx, feed_id, "").await;

            let form_state_new = FormState {
                feed_id: model.feed.feed_id.clone(),
                tags: vec![],
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

            ctx.feed_tags_form_state_db
                .put(&form_state_new)
                .await
                .unwrap_or(());

            Ok(())
        }

        Route::ClickedGoBack { feed_id: _ } => Ok(()),
    }
}

fn route(route: Route) -> String {
    route::Route::Feed(feed::route::Route::Tags(route)).encode()
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
    route::Route::Feed(feed::route::Route::Screen { feed_id })
}

fn view_search_input(feed_id: &FeedId) -> Elem {
    SearchBar::new()
        .url(&route(Route::InputtedSearch {
            feed_id: feed_id.clone(),
        }))
        .indicator("signalIsSearching")
        .input_id(&to_screen_id(feed_id, "search-input"))
        .input_model("signalInputValue")
        .placeholder("Search tags")
        .view()
}

fn js_signal_is_checked(tag: &FeedTag) -> String {
    format!(
        "$signalSelectedTagIds.map(v => v.toLowerCase().trim()).includes('{}')",
        tag.encode().to_lowercase()
    )
}

fn view_selected_root(feed_id: &FeedId) -> Elem {
    div().id(&to_screen_id(feed_id, "selected")).class(
        "flex-none flex flex-row items-center justify-start p-4 gap-2 min-h-20 flex-wrap border-b",
    )
}

fn view_selected_loading(feed_id: &FeedId) -> Elem {
    view_selected_root(feed_id).child(div().class("text-muted").child_text("Loading..."))
}

fn view_selected(model: &ViewModel) -> Elem {
    view_selected_root(&model.feed.feed_id)
        .child(
            div()
                .data_show("($signalSelectedTagIds).length === 0")
                .class("text-muted")
                .child_text("No tags selected"),
        )
        .children(
            model
                .to_all_tags()
                .iter()
                .map(|tag| {
                    tag.chip()
                        .size(ChipSize::Small)
                        .signal_checked(&js_signal_is_checked(tag))
                        .view()
                        .data_show(&js_signal_is_checked(tag))
                        .data_on_clicked_tag(&tag.encode())
                })
                .collect::<Vec<Elem>>(),
        )
        .child(
            button()
                .data_show("($signalSelectedTagIds).length > 0")
                .data_on(|b| {
                    b.click()
                        .js("$signalSelectedTagIds = []")
                        .patch(&route(Route::ClickedClear {
                            feed_id: model.feed.feed_id.clone(),
                        }))
                })
                .class("underline text-secondary p-2")
                .child_text("Clear"),
        )
    // .child(
    //     div()
    //         .class("px-1 flex gap-1")
    //         .data_show("$signalIsUpatingSelected")
    //         .child(spinner("size-6 animate-spin")), // .child_text("Loading..."),
    // )
}

fn view_screen(feed_id: &FeedId) -> Elem {
    div()
        .id(&to_screen_id(feed_id, ""))
        .data_store(
            r#"{
                signalInputValue: '', 
                signalUnselectedTagIds: [],  
                signalSelectedTagIds: [],
            }"#,
        )
        .data_on(|b| b
            .e("clicked-tag")
            .js("const v = evt?.detail?.tagId?.toLowerCase?.()?.trim?.()")
            .js("$signalSelectedTagIds = $signalSelectedTagIds.map(v => v.toLowerCase().trim())")
            .js("$signalSelectedTagIds = $signalSelectedTagIds.includes(v) ? $signalSelectedTagIds.filter(v_ => v_ !== v) : [...$signalSelectedTagIds, v]")
            .patch(&route(Route::ClickedTag {feed_id: feed_id.clone()}))
        )
        .data_indicator("signalIsUpatingSelected")
        .class("w-full h-full flex flex-col overflow-hidden relative")
        .child(view_selected_loading(feed_id))
        .child(view_search_input(feed_id))
        .child(view_unselected_loading(feed_id))
        .child(view_bottom_bar(feed_id))
}

impl Elem {
    fn data_on_clicked_tag(self, tag_id: &str) -> Self {
        self.data_on(|b| {
            b.click()
                .js(&format!("const tagId = '{}'", tag_id))
                .js("const e = new CustomEvent('clicked-tag', { bubbles: true, detail: { tagId } })")
                .js("evt.target.dispatchEvent(e)")
        })
    }
}

fn view_bottom_bar(feed_id: &FeedId) -> Elem {
    div()
        .id(&to_screen_id(feed_id, "bottom-bar"))
        .class("flex-none flex flex-row items-center justify-center p-4 border-t gap-4 min-h-20")
        .child(
            Button::new()
                .label("Cancel")
                .color(Color::Gray)
                .view()
                .data_on(|b| {
                    b.click()
                        .push_then_get(&to_back_route(feed_id.clone()).encode())
                })
                .type_("button")
                .class("flex-1"),
        )
        .child(
            Button::new()
                .label("Save")
                .color(ui::button::Color::Primary)
                .indicator("signalIsSaving")
                .view()
                .data_on(|b| {
                    b.click().get(&route(Route::ClickedSave {
                        feed_id: feed_id.clone(),
                    }))
                })
                .id("save-button")
                .class("flex-1"),
        )
}

fn view_unselected_root(feed_id: &FeedId) -> Elem {
    div()
        .id(&to_screen_id(feed_id, "unselected"))
        .class("flex-1 flex flex-col p-4 pt-5 overflow-y-auto overflow-x-hidden")
}

fn view_unselected_loading(feed_id: &FeedId) -> Elem {
    view_unselected_root(feed_id).child(spinner_page::view())
}

fn view_unselected(model: &ViewModel) -> Elem {
    view_unselected_root(&model.feed.feed_id).child(view_search_results_content(&model))
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
                    .signal_checked(&js_signal_is_checked(feed_tag))
                    .view()
                    .data_on_clicked_tag(&feed_tag.encode())
            })
            .collect::<Vec<Elem>>(),
    )
}
