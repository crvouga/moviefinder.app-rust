use crate::{
    core::{html::*, hx, ui},
    route::{self},
};

use super::root::ROOT_SELECTOR;

pub fn root(attrs: &[Attr], children: &[Elem]) -> Elem {
    div(
        [class("flex items-center justify-center w-full border-b h-16 font-bold text-lg text-center truncate")].into_iter().chain(attrs.iter().cloned()).collect::<Vec<Attr>>().as_ref(),
        children,
    )
}

pub fn back_button(back_route: route::Route) -> Elem {
    button(
        &[
            class("size-16 flex items-center justify-center"),
            hx::target(ROOT_SELECTOR),
            hx::Swap::InnerHtml.into(),
            hx::get(&back_route.encode()),
            hx::push_url("true"),
            hx::Preload::MouseDown.into(),
            aria_label("Go back"),
        ],
        &[ui::icon::back_arrow(&[class("size-8")])],
    )
}

pub fn cancel_button(back_route: route::Route) -> Elem {
    button(
        &[
            class("size-16 flex items-center justify-center"),
            hx::target(ROOT_SELECTOR),
            hx::Swap::InnerHtml.into(),
            hx::get(&back_route.encode()),
            hx::push_url("true"),
            hx::Preload::MouseDown.into(),
            aria_label("Cancel"),
        ],
        &[ui::icon::x_mark(&[class("size-8")])],
    )
}

pub fn title(title: &str) -> Elem {
    div(
        &[class(
            "flex-1 text-center flex items-center justify-center h-full truncate",
        )],
        &[text(title)],
    )
}

pub fn empty() -> Elem {
    div(&[class("size-16")], &[])
}
