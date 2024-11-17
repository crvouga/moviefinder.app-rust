use super::route::Route;
use crate::{
    core::{
        html::*,
        http::form_data::FormData,
        res::Res,
        ui::{
            self,
            button::{Button, Color},
            chip::ChipSize,
            icon, spinner_page,
        },
    },
    ctx::Ctx,
    feed::{self, core::Feed, feed_filter::FeedFilter, feed_id::FeedId},
    req::Req,
    route,
    ui::top_bar,
};

#[derive(Debug)]
struct ViewModel {
    feed: Feed,
    filters: Vec<FeedFilter>,
}

const FEED_FILTER_ID_KEY: &str = "genre_id";

pub async fn respond(ctx: &Ctx, req: &Req, feed_id: &FeedId, route: &Route) -> Res {
    match route {
        Route::IndexLoad => {
            let res: Res = view_load_index(&feed_id).into();

            res.cache()
        }

        Route::Index => {
            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let genres = ctx.genre_db.get_all().await.unwrap_or(vec![]);

            let filters: Vec<FeedFilter> = genres
                .iter()
                .map(|genre| FeedFilter::Genre(genre.clone()))
                .collect();

            let view_model = ViewModel { feed, filters };

            view_index(&view_model).into()
        }

        Route::ClickedSave => {
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let feed_filters_new: Vec<FeedFilter> = req.form_data.clone().into();

            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let feed_new = Feed {
                start_index: 0,
                filters: feed_filters_new,
                ..feed
            };

            ctx.feed_db.put(feed_new.clone()).await.unwrap_or(());

            Res::root_redirect_screen(to_back_route(feed_new.feed_id))
        }
    }
}

fn to_back_route(feed_id: FeedId) -> route::Route {
    route::Route::Feed(feed::route::Route::IndexLoad { feed_id })
}

impl From<FormData> for Vec<FeedFilter> {
    fn from(form_data: FormData) -> Self {
        form_data
            .get_all(FEED_FILTER_ID_KEY)
            .cloned()
            .unwrap_or(vec![])
            .into_iter()
            .filter_map(|encoded| {
                let decoded = FeedFilter::decode(&encoded);
                println!("encoded {:?}, decoded: {:?}", &encoded, &decoded);
                decoded
            })
            .collect()
    }
}

const INDEX_ID: &str = "feed-controls";
fn index_selector() -> String {
    format!("#{}", INDEX_ID)
}

fn view_top_bar(feed_id: &FeedId, loading_path: &str) -> Elem {
    top_bar::view_root()
        // .child(view_close_button(feed_id, loading_path))
        .child(
            div()
                .class("flex-1 h-full")
                .child(view_search_input(feed_id, loading_path)),
        )
}

fn view_search_input(feed_id: &FeedId, loading_path: &str) -> Elem {
    div()
        .class("w-full h-full relative")
        .x_data("{search:''}")
        .child(
            div()
                .class("absolute top-1/2 left-5 transform -translate-y-1/2")
                .child(icon::magnifying_glass("size-6")),
        )
        .child(
            button()
                .x_show("search.length > 0")
                .x_on("click", "search = ''; $refs.search.focus();")
                .type_("button")
                .tab_index(0)
                .aria_label("clear search")
                .class("absolute top-1/2 right-0 size-16 grid place-items-center transform -translate-y-1/2")
                .child(icon::x_mark("size-6")),
        )
        .child(
            button()
                .x_show("search.length === 0")
                .type_("button")
                .tab_index(0)
                .aria_label("close")
                .hx_loading_disabled()
                .hx_loading_path(loading_path)
                .hx_abort(&index_selector())
                .root_push_screen(to_back_route(feed_id.clone()))
                .class("absolute top-1/2 right-0 size-16 grid place-items-center transform -translate-y-1/2")
                .child(icon::chevron_down("size-6")),
        )
        .child(
            input()
                .x_model("search")
                .x_ref("search")
                .class("w-full h-full bg-transparent")
                .class("pl-14")
                .class("pr-14")
                .placeholder("Search"),
        )
}

fn view_load_index(feed_id: &FeedId) -> Elem {
    div()
        .class("w-full h-full flex flex-col overflow-hidden relative")
        .root_swap_screen(route::Route::Feed(feed::route::Route::Controls {
            feed_id: feed_id.clone(),
            child: Route::Index,
        }))
        .hx_trigger_load()
        .id(INDEX_ID)
        .child(view_top_bar(&feed_id, ""))
        .child(spinner_page::view())
        .child(view_bottom_bar(&feed_id, ""))
}

fn view_index(view_model: &ViewModel) -> Elem {
    let clicked_save_path = route::Route::Feed(feed::route::Route::Controls {
        feed_id: view_model.feed.feed_id.clone(),
        child: Route::ClickedSave,
    })
    .encode();
    form()
        .class("w-full h-full flex flex-col overflow-hidden relative")
        .hx_post(&clicked_save_path)
        .hx_swap_none()
        .child(view_top_bar(&view_model.feed.feed_id, &clicked_save_path))
        .child(view_body(view_model))
        .child(view_bottom_bar(
            &view_model.feed.feed_id,
            &clicked_save_path,
        ))
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

fn view_body(view_model: &ViewModel) -> Elem {
    div()
        .class("flex-1 flex flex-col p-4 pt-5 overflow-y-auto")
        .child(view_chips(&view_model))
}

fn view_chips(view_model: &ViewModel) -> Elem {
    let mut active: Vec<FeedFilter> = view_model.feed.clone().filters;
    active.sort();

    let mut inactive: Vec<FeedFilter> = view_model
        .filters
        .clone()
        .into_iter()
        .filter(|filter| !active.contains(filter))
        .collect();
    inactive.sort();

    div()
        .class("flex flex-row items-start justify-start flex-wrap gap-2")
        .child(view_chips_frag(active, true))
        .child(view_chips_frag(inactive, false))
}

fn view_chips_frag(feed_filters: Vec<FeedFilter>, is_checked: bool) -> Elem {
    frag().children(
        feed_filters
            .iter()
            .map(|filter| {
                filter
                    .chip()
                    .size(ChipSize::Large)
                    .checked(is_checked)
                    .disabled(false)
                    .name(FEED_FILTER_ID_KEY)
                    .view()
            })
            .collect::<Vec<Elem>>(),
    )
}
