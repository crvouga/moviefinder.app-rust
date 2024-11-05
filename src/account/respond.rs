use super::route::Route;
use crate::{
    core::{html::*, res::Res},
    ui::bottom_bar,
};

pub fn respond(route: &Route) -> Res {
    match route {
        Route::Index => {
            let res: Res = view_login_cta().into();
            res.cache()
        }
    }
}

pub fn view_login_cta() -> Elem {
    div()
        .class("w-full flex-1 flex items-center justify-center flex-col")
        .child(div().class("flex-1").child_text("Account"))
        .child(bottom_bar::view(bottom_bar::Active::Account, &""))
}
