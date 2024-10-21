use crate::core::html::*;

pub fn error(label: &str) -> Elem {
    elem(
        "div",
        &[class("relative flex w-full items-center justify-start rounded border border-red-400 bg-red-800 px-4 py-3 text-white")],
        &[text(label)],
    )
}
