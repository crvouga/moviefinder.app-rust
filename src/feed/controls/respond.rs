use super::{ctx::Ctx, form_state::FormState, route::Route, view_model::ViewModel};
use crate::{
    core::{
        html::*,
        params::Params,
        ui::{
            self,
            button::{Button, Color},
            chip::ChipSize,
            icon::{self, spinner},
            spinner_page,
        },
    },
    feed::{self, feed_::Feed, feed_id::FeedId, feed_tag::FeedTag},
    req::Req,
    res::Res,
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

pub async fn respond(ctx: &Ctx, req: &Req, feed_id: &FeedId, route: &Route) -> Res {
    match route {
        Route::IndexLoad => {
            let res: Res = view_load_index(&feed_id).into();

            res.cache()
        }

        Route::Index => {
            let model = ViewModel::load(ctx, feed_id, "").await;
            view_index(&model).into()
        }

        Route::ClickedSave => {
            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let form_state = FormState::load(ctx, &feed).await;

            let feed_new = Feed {
                start_index: 0,
                tags: form_state.tags,
                ..feed
            };

            ctx.feed_db.put(feed_new.clone()).await.unwrap_or(());

            Res::root_redirect(to_back_route(feed_new.feed_id))
        }

        Route::ClickedTag { tag } => {
            let model = ViewModel::load(ctx, feed_id, "").await;

            let form_state_new = model.form_state.toggle(tag);

            ctx.form_state_db.put(&form_state_new).await.unwrap_or(());

            Res::empty()
        }

        Route::InputtedSearch => {
            let default = "".to_string();

            let search_input = req.params.get_first(SEARCH_NAME).unwrap_or(&default);

            let model = ViewModel::load(ctx, feed_id, search_input).await;

            view_tags(&model).into()
        }

        Route::ClickedGoBack => Res::empty(),
    }
}

fn to_back_route(feed_id: FeedId) -> route::Route {
    route::Route::Feed(feed::route::Route::IndexLoad { feed_id })
}

fn view_root() -> Elem {
    div().class("w-full h-full flex flex-col overflow-hidden relative")
}

fn view_search_bar(feed_id: &FeedId, loading_path: &str) -> Elem {
    let inputted_search_path = route::Route::Feed(feed::route::Route::Controls {
        feed_id: feed_id.clone(),
        child: Route::InputtedSearch,
    })
    .encode();
    label()
        .hx_post(&inputted_search_path)
        .hx_trigger("load, input delay:300ms from:input, cleared from:input")
        .hx_target(&tags_selector())
        .hx_swap_outer_html()
        .hx_loading_aria_busy()
        .hx_include_this()
        .hx_loading_path(&inputted_search_path)
        .class("w-full h-16 shrink-0 border-b group flex items-center gap-2 overflow-hidden")
        .child(
            div()
                .class("h-full grid place-items-center pl-4 pr-2")
                .child(icon::magnifying_glass("size-6")),
        )
        .child(
            input()
                .id("feed-controls-search-input")
                .hx_preserve_state("feed-controls-search-input")
                .class("flex-1 h-full bg-transparent peer outline-none")
                .type_("text")
                .name(SEARCH_NAME)
                .hx_on(
                    "clear",
                    "this.value = ''; this.focus(); this.dispatchEvent(new Event('cleared'))",
                )
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
                .root_push_route(to_back_route(feed_id.clone()))
                .class("h-full pr-5 place-items-center")
                .class("hidden peer-placeholder-shown:grid")
                .child(icon::x_mark("size-6")),
        )
        .child(
            button()
                .type_("button")
                .hx_on(
                    "click",
                    "this.parentElement.querySelector('input').dispatchEvent(new Event('clear'));",
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
                .root_swap_route(route::Route::Feed(feed::route::Route::Controls {
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
                .root_push_route(to_back_route(feed_id.clone()))
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
                    // .hx_on("click", "console.log('hello');document.getElementById('feed-controls-search-input').dispatchEvent(new Event('clear'));")
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
