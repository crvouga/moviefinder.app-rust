use super::{list_::MediaInteractionList, route::Route};
use crate::{
    core::{
        html::{div, Elem},
        http::response_writer::ResponseWriter,
    },
    ctx::Ctx,
    list::list::view_lists,
    req::Req,
    ui::route::AppRoute,
    user::user_id::UserId,
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

            w.send_fragment(view_interaction_lists(Some(interaction_lists)))
                .await?;

            Ok(())
        }

        Route::ListScreen { user_id, name } => {
            println!("{:?} {:?} ", user_id, name);

            let list_item = ctx
                .media_interaction_list_item_db
                .find_by_user_id_and_interaction_name(0, 100, user_id.clone(), name.clone())
                .await?;

            println!("{:?} ", list_item);

            Ok(())
        }
    }
}

pub fn view_lists_section(user_id: UserId) -> Elem {
    div()
        .id("interaction-lists-section")
        .class("w-full flex flex-col")
        .data_on(|e| e.load().sse(&Route::ListsSection { user_id }.url()))
        .child(view_interaction_lists(None))
        .child(div().class("w-full h-32"))
}

pub fn view_interaction_lists(lists: Option<Vec<MediaInteractionList>>) -> Elem {
    view_lists(lists).id("interaction-lists")
}
