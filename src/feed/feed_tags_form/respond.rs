use super::{form_state::FormState, route::Route, view_model::ViewModel};
use crate::{
    core::{
        dynamic_data::DynamicData,
        html::*,
        http::response_writer::ResponseWriter,
        ui::{chip::ChipSize, search_bar::SearchBar, spinner_page},
        unit_of_work::UnitOfWork,
    },
    ctx::Ctx,
    feed::{self, feed_::Feed, feed_id::FeedId, feed_screen, feed_tag::FeedTag},
    req::Req,
    ui::{bottom_bar_form::BottomBarForm, route::Routable},
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

            w.send_signal(
                "signal_selected_tags_ids",
                &format!("[{}]", signal_selected_tag_ids),
            )
            .await?;

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

            w.send_signal("signal_is_saving", "false").await?;

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

            let signal_input_value = r.payload.get_first("signal_input_value");

            let model =
                ViewModel::load(ctx, feed_id, &signal_input_value.unwrap_or_default()).await;

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
            let search_input = r
                .payload
                .get_first("signal_input_value")
                .unwrap_or_default();

            let model = ViewModel::load(ctx, feed_id, &search_input).await;

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
        let signal_selected_tag_ids = self.payload.get_all("signal_selected_tags_ids");

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
        .indicator("signal_is_searching")
        .input(|e| {
            e.id("search-input")
                .data_bind("signal_input_value")
                .placeholder("Search tags")
        })
        .view()
}

fn js_signal_is_checked(tag: &FeedTag) -> String {
    format!(
        "signal_selected_tags_ids.value.map(v => v.toLowerCase().trim()).includes('{}')",
        tag.encode().to_lowercase()
    )
}

fn view_selected_root() -> Elem {
    div().id("selected").class(
        "flex-none flex flex-row items-center justify-start p-4 gap-2 min-h-16 flex-wrap border-b",
    )
}

fn view_selected_loading() -> Elem {
    view_selected_root().child(div().class("text-muted").child_text("Loading..."))
}

fn view_selected(model: &ViewModel) -> Elem {
    view_selected_root()
        .child(
            div()
                .data_show("signal_selected_tags_ids.value?.length === 0")
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
                .data_show("signal_selected_tags_ids.value?.length > 0")
                .data_on(|b| {
                    b.click().js("signal_selected_tags_ids.value = []").sse(
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
    form()
        .id("screen")
        .data_signal("signal_input_value", "''")
        .data_signal("signal_selected_tags_ids", "[]")
        .debug_signals(false)
        .data_indicator("signal_is_updating_selected")
        .class("w-full h-full flex flex-col overflow-hidden relative")
        .data_on(|e| e.submit().prevent_default().sse(&Route::ClickedSave { feed_id: feed_id.clone() }.url()))
        .child(
            div()
            .class("w-full h-full flex flex-col overflow-hidden relative")
            .data_on(|b| b
                .e("clicked_tag")
                .js("signal_selected_tags_ids.value = (signal_selected_tags_ids.value).map(x => x.toLowerCase().trim())")
                .js("const js_id = evt?.detail?.js_tag_id?.toLowerCase?.()?.trim?.()")
                .js("const js_ids = signal_selected_tags_ids.value")
                .js("signal_selected_tags_ids.value = js_ids.includes(js_id) ? js_ids.filter(x => x !== js_id) : [...js_ids, js_id]")
                .sse(&Route::ClickedTag {feed_id: feed_id.clone()}.url())
            )
            .child(view_selected_loading())
            .child(view_search_input(feed_id))
            .child(view_unselected_loading())
            .child(view_bottom_bar(feed_id.clone()))
        )
}

impl Elem {
    fn data_on_clicked_tag(self, tag_id: &str) -> Self {
        self.id(tag_id).data_on(|b| {
            b.click()
                .js("const js_tag_id = evt.target.id")
                .js("const e = new CustomEvent('clicked_tag', { bubbles: true, detail: { js_tag_id } })")
                .js("evt.target.dispatchEvent(e)")
        })
    }
}

fn view_bottom_bar(feed_id: FeedId) -> Elem {
    BottomBarForm::default()
        .on_cancel(move |e| {
            e.click()
                .push_then_sse(&feed::route::Route::FeedScreen { feed_id }.url())
        })
        .view()
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
