use crate::core::html::{div, Elem};

use super::icon;

pub fn view() -> Elem {
    div()
        .class("w-full h-full flex items-center justify-center")
        .child(icon::spinner("size-16 animate-spin"))
}
