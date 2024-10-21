use crate::html::*;
use crate::ui;

pub fn page() -> Elem {
    div(
        &[class("w-full h-full flex items-center justify-center")],
        &[ui::icon::spinner(&[class("size-16 animate-spin")])],
    )
}
