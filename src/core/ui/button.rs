use crate::core::html::*;

use super::icon::spinner;

#[derive(Debug, Default)]
pub struct Button {
    label: String,
    color: Color,
}

#[derive(Debug, Default)]
pub enum Color {
    #[default]
    Primary,
    Gray,
}

impl Button {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn view(self) -> Elem {
        let mut base_classes = "group relative flex items-center justify-center gap-2 rounded px-4 py-3 text-lg font-bold text-white".to_string();

        base_classes.push_str(" enabled:hover:opacity-80 enabled:active:opacity-60");

        match self.color {
            Color::Gray => {
                base_classes.push_str(" bg-neutral-800");
            }
            Color::Primary => {
                base_classes.push_str(" bg-blue-600");
            }
        }

        button()
        .class("group relative flex items-center justify-center gap-2 rounded px-4 py-3 text-lg font-bold text-white")
        .class("enabled:hover:opacity-80 enabled:active:opacity-60")
        .class(
            match self.color {
                Color::Gray => "bg-neutral-800",                
                Color::Primary => "bg-blue-600"
            }
        )
        .attr("data-loading-aria-busy", "")
        .attr("data-loading-disable", "")
        .child(
            div().class("absolute inset-0 flex items-center justify-center opacity-0 group-aria-busy:opacity-100")
    .child(spinner("size-8 animate-spin"))
        )
        .child(
            div()
        .class("group-aria-busy:invisible")
        .child_text(&self.label)
        )
    }
}
