use super::{form_state::FormState, route::Route, view_model::ViewModel};
use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        params::Params,
        ui::{
            self,
            button::{Button, Color},
            chip::ChipSize,
            search_bar::SearchBar,
            spinner_page,
        },
        unit_of_work::UnitOfWork,
    },
    ctx::Ctx,
    feed::{self, feed_::Feed, feed_id::FeedId, feed_screen, feed_tag::FeedTag},
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
        Route::FeedTagsFormScreen { feed_id } => {
            w.send_screen(view_screen(&feed_id)).await?;

            let model = ViewModel::load(ctx, feed_id, "").await;

            let signal_selected_tag_ids = model
                .form_state
                .tags
                .iter()
                .map(|t| format!("'{}'", t.encode().to_lowercase()))
                .collect::<Vec<String>>()
                .join(",");

            let signals_new = &format!("{{signalSelectedTagIds: [{}]}}", signal_selected_tag_ids);

            w.send_signals(signals_new).await?;

            w.send_fragment(view_selected(&model)).await?;

            w.send_fragment(view_unselected(&model)).await?;

            Ok(())
        }

        Route::ClickedSave { feed_id } => {
            let model = ViewModel::load(ctx, feed_id, "").await;

            let feed_new = Feed {
                start_index: 0,
                tags: model.form_state.tags,
                ..model.feed
            };

            UnitOfWork::transact(|uow| async move {
                ctx.feed_db.put(uow.clone(), feed_new.clone()).await?;
                Ok(())
            })
            .await?;

            w.send_signals("{signalIsSaving: false}").await?;

            w.send_push_url(
                &feed::route::Route::FeedScreen {
                    feed_id: feed_id.clone(),
                }
                .url(),
            )
            .await?;

            feed_screen::respond(ctx, r, w, feed_id).await?;

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

            w.send_fragment(view_selected(&model_new)).await?;

            w.send_fragment(view_unselected(&model_new)).await?;

            let uow = UnitOfWork::new();

            ctx.feed_tags_form_state_db
                .put(uow, &form_state_new)
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

            w.send_fragment(view_unselected(&model_new)).await?;

            w.send_fragment(view_selected(&model_new)).await?;

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

            w.send_fragment(view_selected(&model_new)).await?;

            let uow = UnitOfWork::new();

            ctx.feed_tags_form_state_db
                .put(uow, &form_state_new)
                .await
                .unwrap_or(());

            Ok(())
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

fn view_search_input(feed_id: &FeedId) -> Elem {
    SearchBar::default()
        .url(
            &Route::InputtedSearch {
                feed_id: feed_id.clone(),
            }
            .url(),
        )
        .indicator("signalIsSearching")
        .input(|e| {
            e.id("search-input")
                .data_model("signalInputValue")
                .placeholder("Search tags")
        })
        .view()
}

fn js_signal_is_checked(tag: &FeedTag) -> String {
    format!(
        "$signalSelectedTagIds.map(v => v.toLowerCase().trim()).includes('{}')",
        tag.encode().to_lowercase()
    )
}

fn view_selected_root() -> Elem {
    div().id("selected").class(
        "flex-none flex flex-row items-center justify-start p-4 gap-2 min-h-20 flex-wrap border-b",
    )
}

fn view_selected_loading() -> Elem {
    view_selected_root().child(div().class("text-muted").child_text("Loading..."))
}

fn view_selected(model: &ViewModel) -> Elem {
    view_selected_root()
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
                    b.click().js("$signalSelectedTagIds = []").patch(
                        &Route::ClickedClear {
                            feed_id: model.feed.feed_id.clone(),
                        }
                        .url(),
                    )
                })
                .class("underline text-secondary p-2")
                .child_text("Clear"),
        )
}

fn view_screen(feed_id: &FeedId) -> Elem {
    div()
        .id("screen")
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
            .patch(&Route::ClickedTag {feed_id: feed_id.clone()}.url())
        )
        .data_indicator("signalIsUpdatingSelected")
        .class("w-full h-full flex flex-col overflow-hidden relative")
        .child(view_selected_loading())
        .child(view_search_input(feed_id))
        .child(view_unselected_loading())
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
        .id("bottom-bar")
        .class("flex-none flex flex-row items-center justify-center p-4 border-t gap-4 min-h-20")
        .child(
            Button::default()
                .label("Cancel")
                .color(Color::Gray)
                .view()
                .data_on(|b| {
                    b.click().push_then_get(
                        &feed::route::Route::FeedScreen {
                            feed_id: feed_id.clone(),
                        }
                        .url(),
                    )
                })
                .type_("button")
                .class("flex-1"),
        )
        .child(
            Button::default()
                .label("Save")
                .color(ui::button::Color::Primary)
                .indicator("signalIsSaving")
                .view()
                .data_on(|b| {
                    b.click().get(
                        &(Route::ClickedSave {
                            feed_id: feed_id.clone(),
                        })
                        .url(),
                    )
                })
                .id("save-button")
                .class("flex-1"),
        )
}

fn view_unselected_root() -> Elem {
    div()
        .id("unselected")
        .class("flex-1 flex flex-col p-4 pt-5 overflow-y-auto overflow-x-hidden")
}

fn view_unselected_loading() -> Elem {
    view_unselected_root().child(spinner_page::view())
}

fn view_unselected(model: &ViewModel) -> Elem {
    view_unselected_root().child(view_search_results_content(&model))
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
