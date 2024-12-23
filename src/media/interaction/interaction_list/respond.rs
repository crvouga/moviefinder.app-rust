use super::{list_::MediaInteractionList, route::Route};
use crate::{
    core::{
        html::{div, Elem},
        http::response_writer::ResponseWriter,
    },
    ctx::Ctx,
    list::list::{list_screen::ListScreen, list_section::ListSection},
    req::Req,
    ui::route::AppRoute,
    user::{self, user_id::UserId},
};

pub async fn respond(
    ctx: &Ctx,
    _r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::ListsSection { user_id } => {
            let interaction_lists = ctx
                .media_interaction_list_db
                .find_by_user_id(user_id.clone())
                .await?;

            w.send_fragment(view_lists_section(user_id.clone(), Some(interaction_lists)))
                .await?;

            Ok(())
        }

        Route::ListScreen { user_id, name } => {
            let media_interaction_list = MediaInteractionList {
                interaction_name: name.clone(),
                user_id: user_id.clone(),
            };

            let list_screen = ListScreen::new(media_interaction_list)
                .back_url(user::route::Route::AccountScreen.url());

            w.send_screen(list_screen.clone().view()).await?;

            let _list_items = ctx
                .media_interaction_list_item_db
                .find_by_user_id_and_interaction_name(0, 10_000, user_id.clone(), name.clone())
                .await?;

            w.send_screen(list_screen.clone().view()).await?;

            Ok(())
        }
    }
}

pub fn view_lists_section(user_id: UserId, lists: Option<Vec<MediaInteractionList>>) -> Elem {
    div()
        .id("interaction-lists-section")
        .class("w-full flex flex-col")
        .map(|e| match lists {
            None => e.data_on(|e| e.load().sse(&Route::ListsSection { user_id }.url())),
            Some(_) => e,
        })
        .child(ListSection::default().lists(lists).view())
        .child(div().class("w-full h-32"))
}
