use super::alert::Alert;
use crate::core::html::*;

pub fn page(label: &str) -> Elem {
    div()
        .class("flex h-full w-full items-center justify-center p-8")
        .child(Alert::error().label(label).view())
}
