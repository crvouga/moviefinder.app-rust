use super::route::Route;
use crate::{
    core::{
        html::{div, Elem},
        http::response_writer::ResponseWriter,
        ui::icon::solid::spinner,
    },
    ctx::Ctx,
    list::list::List,
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
        Route::Lists { user_id } => {
            let interaction_lists = ctx
                .media_interaction_list_db
                .find_by_user_id(user_id.clone())
                .await?;

            let lists: Vec<List> = interaction_lists.into_iter().map(|l| l.into()).collect();

            w.send_fragment(view_lists(Some(lists))).await?;

            Ok(())
        }

        Route::List { user_id, name } => {
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
        .data_on(|e| e.load().sse(&Route::Lists { user_id }.url()))
        .child(view_lists(None))
}

pub fn view_lists(lists: Option<Vec<List>>) -> Elem {
    div()
        .id("interaction-lits")
        .class("w-full flex flex-col")
        .child(match lists {
            None => div()
                .class("w-full flex items-center justify-center")
                .child(spinner("size-12 animate-spin")),

            Some(lists) => div().children(
                lists
                    .into_iter()
                    .map(|list| div().child_text(&list.name))
                    .collect(),
            ),
        })
}
