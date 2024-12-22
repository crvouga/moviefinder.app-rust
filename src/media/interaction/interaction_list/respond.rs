use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

use super::route::Route;

pub async fn respond(
    _ctx: &Ctx,
    _r: &Req,
    route: &Route,
    _w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Lists { user_id } => {
            println!("{:?} ", user_id);
            Ok(())
        }

        Route::List { user_id, name } => {
            println!("{:?} {:?} ", user_id, name);
            Ok(())
        }
    }
}
