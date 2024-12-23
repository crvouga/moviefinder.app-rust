use super::{list_screen_db::MediaListScreenDb, route::Route};
use crate::{
    core::{
        html::{div, p, Elem},
        http::response_writer::ResponseWriter,
        pagination::{Paginated, Pagination},
        remote_result::RemoteResult,
        ui::{alert::Alert, list::List, spinner_block::SpinnerBlock, top_bar::TopBar},
    },
    ctx::Ctx,
    list::{list::MediaList, list_item::MediaListItem},
    req::Req,
};

use std::fmt::Debug;

pub async fn respond<TList: MediaList + Debug>(
    list_screen_db: &impl MediaListScreenDb<TList>,
    _ctx: &Ctx,
    _r: &Req,
    route: &Route<TList>,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::IntersectedBottom { list } => {
            println!("IntersectedBottom: {:?}", list);
            Ok(())
        }
        Route::Screen { list, back_url } => {
            let model = ViewModel {
                back_url: Some(back_url.clone()),
                list: Some(list.clone()),
            };

            w.send_screen(view(model)).await?;

            let found = list_screen_db
                .find_list_items(
                    Pagination {
                        limit: 10,
                        offset: 0,
                    },
                    list.to_owned(),
                )
                .await;

            w.send_fragment(view_list_items(found.into())).await?;

            Ok(())
        }
    }
}

#[derive(Clone, Default)]
pub struct ViewModel<TList: MediaList> {
    pub list: Option<TList>,
    pub back_url: Option<String>,
}

pub fn view<T: MediaList>(model: ViewModel<T>) -> Elem {
    let list = model.list.clone();
    let name = list.clone().map(|l| l.name());
    let art = list.map(|l| l.view_art("size-32 rounded shrink-0"));

    div()
        .class("w-full h-full flex flex-col")
        .child(
            TopBar::default()
                .back_url(model.back_url.unwrap_or("".to_owned()))
                .title(&name.clone().unwrap_or_default())
                .view(),
        )
        .child(
            div()
                .class("flex flex-col gap-6 w-full flex-1")
                .child(
                    div()
                        .class("w-full flex items-center justify-center pt-12 flex-col gap-6 px-6")
                        .child(art.unwrap_or_default())
                        .child(
                            p().class("w-full text-center text-3xl font-bold")
                                .child_text(&name.unwrap_or_default()),
                        ),
                )
                .child(view_list_items(RemoteResult::Loading)),
        )
}

fn view_list_items(list_items: RemoteResult<Paginated<MediaListItem>, std::io::Error>) -> Elem {
    div()
        .id("list-items")
        .class("w-full flex flex-col gap-4")
        .child(match list_items {
            RemoteResult::Loading => SpinnerBlock::default().view(),
            RemoteResult::Err(err) => Alert::error().label(&err.to_string()).view(),
            RemoteResult::Ok(list_items) => List::default().view().children(
                list_items
                    .items
                    .into_iter()
                    .map(|_item| div().child_text("hello"))
                    .collect(),
            ),
        })
}
