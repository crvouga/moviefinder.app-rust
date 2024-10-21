use crate::core::html::*;
use crate::core::ui;

pub fn page(attrs: &[Attr]) -> Elem {
    let base_attrs = &[class("w-full h-full flex items-center justify-center")];
    let combined_attrs = [base_attrs, attrs].concat();

    div(
        &combined_attrs,
        &[ui::icon::spinner(&[class("size-16 animate-spin")])],
    )
}
