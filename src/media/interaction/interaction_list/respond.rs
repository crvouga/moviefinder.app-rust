use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

use super::route::Route;

pub async fn respond(
    ctx: &Ctx,
    _r: &Req,
    route: &Route,
    _w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Lists { user_id } => {
            println!("{:?} ", user_id);

            let lists = ctx
                .media_interaction_list_db
                .find_by_user_id(user_id.clone())
                .await?;

            println!("{:?} ", lists);

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
