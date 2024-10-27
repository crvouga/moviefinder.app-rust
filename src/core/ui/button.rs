use crate::core::html::*;

use super::icon::spinner;

pub fn view(label: &str, additional_attrs: Vec<Attr>) -> Elem {
    let base_classes = "group relative flex items-center justify-center gap-2 rounded bg-blue-600 px-4 py-3 text-lg font-bold text-white hover:opacity-80 active:opacity-60";

    let class_str = additional_attrs
        .iter()
        .filter_map(|attr| {
            if attr.name == "class" {
                Some(attr.value.as_str())
            } else {
                None
            }
        })
        .next();

    let combined_class = class_list(&[base_classes, class_str.unwrap_or("")]);

    let mut button_attrs = vec![
        combined_class,
        attr("data-loading-aria-busy", ""),
        attr("data-loading-disable", ""),
    ];
    button_attrs.extend(additional_attrs);

    let spinner_overlay = div(
        &[
            class("absolute inset-0 flex items-center justify-center opacity-0 group-aria-busy:opacity-100"),
        ],
        &[spinner(&[class("size-8 animate-spin")])],
    );

    let label_elem = div(&[class("group-aria-busy:invisible")], &[text(label)]);

    button(&button_attrs, &[spinner_overlay, label_elem])
}
