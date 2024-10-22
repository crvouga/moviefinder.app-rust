use crate::{
    core::{html::*, hx, ui},
    route::{self},
};

use super::root::ROOT_SELECTOR;

pub fn view(back_route: route::Route, title: &str) -> Elem {
    div(
        &[class(
            "flex items-center justify-center w-full border-b h-16 px-4 font-bold text-lg",
        )],
        &[
            button(
                &[
                    class("size-16"),
                    hx::target(&ROOT_SELECTOR),
                    hx::Swap::InnerHtml.into(),
                    hx::get(&back_route.encode()),
                    hx::push_url("true"),
                    hx::Preload::MouseDown.into(),
                    aria_label("Go back"),
                ],
                &[ui::icon::back_arrow(&[class("size-8")])],
            ),
            div(
                &[class("flex-1 flex items-center justify-center h-full")],
                &[text(title)],
            ),
            div(&[class("size-16")], &[]),
        ],
    )
}
