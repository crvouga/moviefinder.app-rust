use super::{list_screen_db::ListScreenDb, route::Route};
use crate::{
    core::{
        html::{div, frag, p, Elem},
        http::response_writer::ResponseWriter,
        pagination::{Paginated, Pagination},
        query::{Query, QueryFilter, QueryOp},
        remote_result::RemoteResult,
        ui::{
            alert::Alert,
            image::Image,
            list::{ViewList, ViewListItem},
            top_bar::TopBar,
        },
    },
    ctx::Ctx,
    list::{list::List, list_item::ListItem, list_item_variant::ListItemVariant},
    media::{media_::Media, media_db::interface::MediaQueryField, media_id::MediaId},
    req::Req,
    ui::route::AppRoute,
};

use std::{fmt::Debug, vec};

pub async fn respond<TList: List + Debug>(
    list_screen_db: &impl ListScreenDb<TList>,
    ctx: &Ctx,
    r: &Req,
    route: &Route<TList>,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
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

            w.send_screen(r, view(model)).await?;

            let found = list_screen_db
                .find_list_items(
                    Pagination {
                        limit: 100,
                        offset: 0,
                    },
                    list.to_owned(),
                )
                .await;

            let list_items = found.clone().into();

            let media_ids = found
                .unwrap_or_default()
                .items
                .clone()
                .iter()
                .filter_map(|item| match item.variant.clone() {
                    ListItemVariant::Media(media_id) => Some(media_id),
                })
                .collect::<Vec<MediaId>>();

            let media = ctx
                .media_db
                .query(Query {
                    limit: media_ids.len() + 1,
                    offset: 0,
                    filter: QueryFilter::Or(
                        media_ids
                            .iter()
                            .map(|id| {
                                QueryFilter::Clause(
                                    MediaQueryField::MediaId,
                                    QueryOp::Eq,
                                    id.as_str().to_owned(),
                                )
                            })
                            .collect(),
                    ),
                })
                .await
                .unwrap_or_default()
                .items;

            let namespace = to_namepsace(Some(list.clone()));

            w.send_fragment(view_list_items(&namespace, list_items, media))
                .await?;

            Ok(())
        }
    }
}

#[derive(Clone, Default)]
pub struct ViewModel<TList: List> {
    pub list: Option<TList>,
    pub back_url: Option<String>,
}

fn to_namepsace<TList: List>(list: Option<TList>) -> String {
    let namespace: String = list.clone().map_or("list".to_string(), |l| {
        let list = l.clone();
        let id = list.id();
        let id_str = id.to_string();
        id_str
    });
    namespace
}

pub fn view<T: List>(model: ViewModel<T>) -> Elem {
    let list = model.list.clone();
    let namespace = to_namepsace(list.clone());
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
                .class("flex flex-col gap-6 w-full flex-1 overflow-y-auto")
                .child(
                    div()
                        .class("w-full flex items-center justify-center pt-12 flex-col gap-6 px-6")
                        .child(art.unwrap_or_default())
                        .child(
                            p().class("w-full text-center text-3xl font-bold")
                                .child_text(&name.unwrap_or_default()),
                        ),
                )
                .child(view_list_items(&namespace, RemoteResult::Loading, vec![])),
        )
}

fn view_list_items(
    namespace: &str,
    list_items: RemoteResult<Paginated<ListItem>, crate::core::error::Error>,
    media: Vec<Media>,
) -> Elem {
    div()
        .id(&format!("list-items-{}", namespace))
        .class("w-full flex flex-col gap-4")
        .child(match list_items {
            RemoteResult::Err(err) => Alert::error().label(&err.to_string()).view(),
            RemoteResult::Loading => ViewList::default().view().children(
                (0..5)
                    .map(|_| {
                        ViewListItem::default()
                            .title("Loading Loading Loading")
                            .art(|_| frag())
                            .skeleton(true)
                            .view()
                    })
                    .collect(),
            ),
            RemoteResult::Ok(list_items) => ViewList::default().view().children(
                list_items
                    .items
                    .into_iter()
                    .filter_map(|item| match item.variant {
                        ListItemVariant::Media(media_id) => {
                            let found = media.iter().find(|m| m.id == media_id).cloned();
                            match found {
                                Some(media) => {
                                    let details_url = media.details_route().url();
                                    Some(
                                        ViewListItem::default()
                                            .title(media.title)
                                            .art(move |class| {
                                                Image::new()
                                                    .view()
                                                    .class(&class)
                                                    .src(media.poster.to_middle_res())
                                            })
                                            .view()
                                            .data_on(|e| e.press_down().push_url(&details_url)),
                                    )
                                }
                                None => Some(
                                    ViewListItem::default()
                                        .title("...".to_owned())
                                        .art(|class| Image::new().view().class(&class))
                                        .view(),
                                ),
                            }
                        }
                    })
                    .collect(),
            ),
        })
}
