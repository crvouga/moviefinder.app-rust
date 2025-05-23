use super::alert::Alert;
use crate::core::html::*;

pub fn screen(label: &str) -> Html {
    div()
        .class("flex h-full w-full items-center justify-center p-8")
        .child(Alert::error().label(label).view())
}
