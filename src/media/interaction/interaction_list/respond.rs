use super::{list_::MediaInteractionList, route::Route};
use crate::{
    core::{
        html::{div, Elem},
        http::response_writer::ResponseWriter,
    },
    ctx::Ctx,
    list::{list_screen, list_section::ListSection},
    req::Req,
    ui::route::AppRoute,
    user::user_id::UserId,
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::ListScreen(route) => list_screen::respond::respond(ctx, r, route, w).await,

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
