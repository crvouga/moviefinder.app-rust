use crate::core::html::*;

use super::icon::spinner;

#[derive(Debug, Default)]
pub struct Props {
    pub label: String,
    pub color: Color,
}

#[derive(Debug)]
pub enum Color {
    Primary,
    Gray,
}

impl Default for Color {
    fn default() -> Self {
        Color::Primary
    }
}

pub fn view(props: Props, additional_attrs: Vec<Attr>) -> Elem {
    let mut base_classes = "group relative flex items-center justify-center gap-2 rounded px-4 py-3 text-lg font-bold text-white hover:opacity-80 active:opacity-60".to_string();

    match props.color {
        Color::Gray => {
            base_classes.push_str(" bg-neutral-800");
        }
        Color::Primary => {
            base_classes.push_str(" bg-blue-600");
        }
    }

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

    let combined_class = class_list(&[&base_classes, class_str.unwrap_or("")]);

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

    let label_elem = div(&[class("group-aria-busy:invisible")], &[text(&props.label)]);

    button(&button_attrs, &[spinner_overlay, label_elem])
}
