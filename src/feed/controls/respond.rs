use std::time::Duration;

use super::route::Route;
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
    ctx::Ctx,
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
    search_input: String,
}

impl ViewModel {
    fn to_tags(&self) -> (Vec<FeedTag>, Vec<FeedTag>) {
        let mut active: Vec<FeedTag> = self
            .feed_tags
            .clone()
            .into_iter()
            .filter(|feed_tag| self.feed.tags.contains(feed_tag))
            .collect();
        active.sort();

        let mut inactive: Vec<FeedTag> = self
            .feed_tags
            .clone()
            .into_iter()
            .filter(|feed_tag| !self.feed.tags.contains(feed_tag))
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

            let model = ViewModel {
                feed,
                feed_tags,
                search_input: "".to_string(),
            };

            view_index(&model).into()
        }

        Route::ClickedSave => {
            let feed_tags_new: Vec<FeedTag> = req
                .form_data
                .get_all(FEED_TAG_ID_NAME)
                .cloned()
                .unwrap_or(vec![])
                .into_iter()
                .filter_map(|encoded| FeedTag::decode(&encoded))
                .collect();

            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let feed_new = Feed {
                start_index: 0,
                tags: feed_tags_new,
                ..feed
            };

            ctx.feed_db.put(feed_new.clone()).await.unwrap_or(());

            Res::root_redirect_screen(to_back_route(feed_new.feed_id))
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

            let model = ViewModel {
                feed,
                feed_tags,
                search_input: search_input.clone(),
            };

            let res: Res = view_tag_chips(&model).into();

            res
        }
    }
}

fn to_back_route(feed_id: FeedId) -> route::Route {
    route::Route::Feed(feed::route::Route::IndexLoad { feed_id })
}

fn view_root() -> Elem {
    div().class("w-full h-full flex flex-col overflow-hidden relative")
}

const SEARCH_BAR_ROOT_ID: &str = "search-bar";
fn search_bar_root_selector() -> String {
    format!("#{}", SEARCH_BAR_ROOT_ID)
}
const SEARCH_BAR_INPUT_ID: &str = "search-input";
fn search_bar_input_selector() -> String {
    format!("#{}", SEARCH_BAR_INPUT_ID)
}

fn view_search_bar(feed_id: &FeedId, loading_path: &str) -> Elem {
    label()
        .id(SEARCH_BAR_ROOT_ID)
        .for_(SEARCH_BAR_INPUT_ID)
        .class("w-full h-16 shrink-0 border-b group flex items-center gap-2 overflow-hidden")
        .hx_loading_aria_busy()
        .child(
            div()
                .class("h-full grid place-items-center pl-4 pr-2")
                .child(icon::magnifying_glass("size-6")),
        )
        .child(
            input()
                .id(SEARCH_BAR_INPUT_ID)
                .class("flex-1 h-full bg-transparent peer outline-none")
                .type_("text")
                .name(SEARCH_NAME)
                .placeholder("Search")
                .hx_post(
                    &route::Route::Feed(feed::route::Route::Controls {
                        feed_id: feed_id.clone(),
                        child: Route::InputtedSearch,
                    })
                    .encode(),
                )
                .hx_target(&tags_selector())
                .hx_trigger_input_changed(Duration::from_millis(300))
                .hx_trigger_focus()
                .hx_swap_inner_html()
                .hx_loading_target(&search_bar_root_selector()),
        )
        .child(
            div()
                .class("group-aria-busy:static group-aria-busy:opacity-100 absolute opacity-0")
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
                .class("h-full pr-5 grid place-items-center")
                .class("opacity-0 absolute peer-placeholder-shown:static peer-placeholder-shown:opacity-100") 
                .child(icon::x_mark("size-6")),
        )
        .child(
            button()
                .type_("button")
                .on_click(&format!("const s = document.getElementById('{}'); s.value = ''; s.focus();", SEARCH_BAR_INPUT_ID))
                .tab_index(0)
                .aria_label("clear search")
                .class("h-full pr-5 grid place-items-center")
                .class("opacity-100 peer-placeholder-shown:opacity-0 peer-placeholder-shown:absolute") 
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
    div()
        .class("flex flex-row items-start justify-start flex-wrap gap-2")
        .child(view_tag_chips_frag(active, true))
        .child(view_tag_chips_frag(inactive, false))
}

fn view_tag_chips_frag(feed_tags: Vec<FeedTag>, is_checked: bool) -> Elem {
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
            })
            .collect::<Vec<Elem>>(),
    )
}
