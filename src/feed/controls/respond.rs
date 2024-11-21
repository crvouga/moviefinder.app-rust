use super::{ctx::Ctx, form_state::FormState, route::Route};
use crate::{
    core::{
        html::*,
        pagination::Paginated,
        query::{Query, QueryFilter, QueryOp},
        res::Res,
        ui::{
            self,
            button::{Button, Color},
            chip::ChipSize,
            icon::{self, spinner},
            spinner_page,
        },
    },
    feed::{
        self, feed_::Feed, feed_id::FeedId, feed_tag::FeedTag,
        feed_tag_db::interface::FeedTagQueryField,
    },
    req::Req,
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

#[derive(Debug)]
struct ViewModel {
    feed: Feed,
    feed_tags: Vec<FeedTag>,
    form_state: FormState,
    search_input: String,
}

impl ViewModel {
    fn to_tags(&self) -> (Vec<FeedTag>, Vec<FeedTag>) {
        let mut active: Vec<FeedTag> = self
            .feed_tags
            .clone()
            .into_iter()
            .filter(|feed_tag| self.form_state.tags.contains(feed_tag))
            .collect();
        active.sort();

        let mut inactive: Vec<FeedTag> = self
            .feed_tags
            .clone()
            .into_iter()
            .filter(|feed_tag| !self.form_state.tags.contains(feed_tag))
            .collect();
        inactive.sort();

        (active, inactive)
    }
}

pub async fn respond(ctx: &Ctx, req: &Req, feed_id: &FeedId, route: &Route) -> Res {
    match route {
        Route::IndexLoad => {
            let res: Res = view_load_index(&feed_id).into();

            res.cache()
        }

        Route::Index => {
            let feed: Feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let feed_tags = ctx
                .feed_tag_db
                .query(Query {
                    offset: 0,
                    limit: 100,
                    filter: QueryFilter::None,
                })
                .await
                .unwrap_or(Paginated::default())
                .items;

            let form_state = FormState::new(&feed);

            ctx.form_state_db.put(&form_state).await.unwrap_or(());

            let model = ViewModel {
                feed,
                feed_tags,
                search_input: "".to_string(),
                form_state,
            };

            view_index(&model).into()
        }

        Route::ClickedSave => {
            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let form_state = get_form_state(ctx, &feed).await;

            let feed_new = Feed {
                start_index: 0,
                tags: form_state.tags,
                ..feed
            };

            ctx.feed_db.put(feed_new.clone()).await.unwrap_or(());

            Res::root_redirect_screen(to_back_route(feed_new.feed_id))
        }

        Route::ClickedTag { tag } => {
            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let mut form_state = get_form_state(ctx, &feed).await;

            if form_state.tags.contains(tag) {
                form_state.tags.retain(|t| t != tag);
            } else {
                form_state.tags.retain(|t| t != tag);
                form_state.tags.push(tag.clone());
            }

            ctx.form_state_db.put(&form_state).await.unwrap_or(());

            Res::empty()
        }

        Route::InputtedSearch => {
            let default = "".to_string();

            let search_input = req.form_data.get_first(SEARCH_NAME).unwrap_or(&default);

            let feed_tags = ctx
                .feed_tag_db
                .query(Query {
                    offset: 0,
                    limit: 100,
                    filter: QueryFilter::Clause(
                        FeedTagQueryField::Label,
                        QueryOp::Like,
                        search_input.clone(),
                    ),
                })
                .await
                .unwrap_or(Paginated::default())
                .items;

            let feed: Feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let form_state = get_form_state(ctx, &feed).await;

            let model = ViewModel {
                feed,
                feed_tags,
                search_input: search_input.clone(),
                form_state,
            };

            let res: Res = view_tag_chips(&model).into();

            res
        }

        Route::ClickedGoBack => Res::empty(),
    }
}

async fn get_form_state(ctx: &Ctx, feed: &Feed) -> FormState {
    let feed_id = feed.feed_id.clone();

    let maybe_form_state = ctx.form_state_db.get(&feed_id).await.unwrap_or(None);

    let mut form_state = maybe_form_state.unwrap_or(FormState::new(feed));
    form_state.feed_id = feed_id;

    form_state
}

fn to_back_route(feed_id: FeedId) -> route::Route {
    route::Route::Feed(feed::route::Route::IndexLoad { feed_id })
}

fn view_root() -> Elem {
    div().class("w-full h-full flex flex-col overflow-hidden relative")
}

fn view_search_bar(feed_id: &FeedId, loading_path: &str) -> Elem {
    label()
        .hx_post(
            &route::Route::Feed(feed::route::Route::Controls {
                feed_id: feed_id.clone(),
                child: Route::InputtedSearch,
            })
            .encode(),
        )
        .hx_target(&tags_selector())
        .hx_trigger("input delay:300ms from:.search-input, focus from:.search-input")
        .hx_swap_inner_html()
        .hx_loading_aria_busy()
        .hx_include_this()
        .class("w-full h-16 shrink-0 border-b group flex items-center gap-2 overflow-hidden")
        .child(
            div()
                .class("h-full grid place-items-center pl-4 pr-2")
                .child(icon::magnifying_glass("size-6")),
        )
        .child(
            input()
                .class("search-input")
                .class("flex-1 h-full bg-transparent peer outline-none")
                .type_("text")
                .name(SEARCH_NAME)
                .placeholder("Search"),
        )
        .child(
            div()
                .class("group-aria-busy:block hidden")
                .child(spinner("size-8 animate-spin")),
        )
        .child(
            button()
                .type_("button")
                .tab_index(0)
                .aria_label("close")
                .hx_loading_disabled()
                .hx_loading_path(loading_path)
                .hx_abort(&index_selector())
                .root_push_screen(to_back_route(feed_id.clone()))
                .class("h-full pr-5 place-items-center")
                .class("hidden peer-placeholder-shown:grid")
                .child(icon::x_mark("size-6")),
        )
        .child(
            button()
                .type_("button")
                .on_click(
                    "const i = this.parentElement.querySelector('input'); i.value = ''; i.focus();",
                )
                .tab_index(0)
                .aria_label("clear search")
                .class("h-full pr-5 place-items-center")
                .class("grid peer-placeholder-shown:hidden")
                .child(icon::x_circle_mark("size-6")),
        )
}

fn view_load_index(feed_id: &FeedId) -> Elem {
    view_root()
        .child(view_search_bar(&feed_id, ""))
        .child(
            spinner_page::view()
                .root_swap_screen(route::Route::Feed(feed::route::Route::Controls {
                    feed_id: feed_id.clone(),
                    child: Route::Index,
                }))
                .hx_trigger_load()
                .id(INDEX_ID),
        )
        .child(view_bottom_bar(&feed_id, ""))
}

fn view_index(model: &ViewModel) -> Elem {
    let clicked_save_path = route::Route::Feed(feed::route::Route::Controls {
        feed_id: model.feed.feed_id.clone(),
        child: Route::ClickedSave,
    })
    .encode();

    view_root()
        .child(view_search_bar(&model.feed.feed_id, &clicked_save_path))
        .child(
            form()
                .class("w-full flex-1 flex flex-col overflow-hidden relative")
                .hx_post(&clicked_save_path)
                .hx_swap_none()
                .child(view_tags(model))
                .child(view_bottom_bar(&model.feed.feed_id, &clicked_save_path)),
        )
}

fn view_bottom_bar(feed_id: &FeedId, loading_path: &str) -> Elem {
    div()
        .class("flex-none flex flex-row items-center justify-center p-4 border-t gap-4")
        .child(
            Button::new()
                .label("Cancel")
                .color(Color::Gray)
                .loading_disabled_path(&loading_path)
                .view()
                .root_push_screen(to_back_route(feed_id.clone()))
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

fn view_tags(model: &ViewModel) -> Elem {
    div()
        .id(TAGS_ID)
        .class("flex-1 flex flex-col p-4 pt-5 overflow-y-auto")
        .child(view_tag_chips(&model))
}

fn view_tag_chips(model: &ViewModel) -> Elem {
    if model.feed_tags.len() == 0 {
        return div()
            .class("w-full overflow-hidden flex-1 flex items-start justify-start font-bold text-lg break-all")
            .child_text(&format!(r#"No results for "{}""#, model.search_input));
    }

    let (active, inactive) = model.to_tags();

    let feed_id = model.feed.feed_id.clone();
    div()
        .class("flex flex-row items-start justify-start flex-wrap gap-2")
        .child(view_tag_chips_frag(&feed_id, active, true))
        .child(view_tag_chips_frag(&feed_id, inactive, false))
}

fn view_tag_chips_frag(feed_id: &FeedId, feed_tags: Vec<FeedTag>, is_checked: bool) -> Elem {
    frag().children(
        feed_tags
            .iter()
            .map(|feed_tag| {
                feed_tag
                    .chip()
                    .size(ChipSize::Large)
                    .checked(is_checked)
                    .disabled(false)
                    .name(FEED_TAG_ID_NAME)
                    .view()
                    .hx_trigger_click()
                    .hx_swap_none()
                    .hx_include_this()
                    .hx_post(
                        &route::Route::Feed(feed::route::Route::Controls {
                            feed_id: feed_id.clone(),
                            child: Route::ClickedTag {
                                tag: feed_tag.clone(),
                            },
                        })
                        .encode(),
                    )
            })
            .collect::<Vec<Elem>>(),
    )
}
