use super::{list_::MediaInteractionList, route::Route};
use crate::{
    core::{
        html::{div, Elem},
        http::response_writer::ResponseWriter,
        pagination::{Paginated, Pagination},
    },
    ctx::Ctx,
    list::{
        list_item::MediaListItem,
        list_screen::{self, list_screen_db::MediaListScreenDb},
        list_section::ListSection,
    },
    req::Req,
    ui::route::AppRoute,
    user::user_id::UserId,
};
use async_trait::async_trait;

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    match route {
        Route::ListScreen(route) => list_screen::respond::respond(ctx, ctx, r, route, w).await,

        Route::ListsSection { user_id } => {
            let interaction_lists = ctx
                .media_interaction_list_db
                .find_by_user_id(user_id.clone())
                .await?;

            w.send_fragment(view_lists_section(user_id.clone(), Some(interaction_lists)))
                .await?;

            Ok(())
        }
    }
}

#[async_trait]
impl MediaListScreenDb<MediaInteractionList> for Ctx {
    async fn find_list_items(
        &self,
        pagination: Pagination,
        list: MediaInteractionList,
    ) -> Result<Paginated<MediaListItem>, crate::core::error::Error> {
        self.media_interaction_list_item_db
            .find_by_user_id_and_interaction_name(pagination, list.user_id, list.interaction_name)
            .await
    }
}

pub fn view_lists_section(user_id: UserId, lists: Option<Vec<MediaInteractionList>>) -> Elem {
    div()
        .id("lists-section")
        .class("w-full flex flex-col")
        .map(|e| match lists {
            None => e.data_on(|e| e.load().sse(&Route::ListsSection { user_id }.url())),
            Some(_) => e,
        })
        .child(ListSection::default().lists(lists).view())
        .child(div().class("w-full h-32"))
}
