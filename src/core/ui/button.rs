use super::icon;
use crate::core::html::*;

#[derive(Debug, Default)]
pub struct Button {
    label: String,
    color: Color,
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
        button()
        .class("group relative flex items-center justify-center gap-2 rounded px-4 py-3 text-lg font-bold text-white")
        .class("enabled:hover:opacity-80 enabled:active:opacity-60")
        .class(&self.color.to_class())
        .hx_loading_aria_busy()
        .hx_loading_disabled()
        .child(
            div()
                .class("absolute inset-0 flex items-center justify-center opacity-0 group-aria-busy:opacity-100")
                .child(icon::spinner("size-8 animate-spin"))
        )
        .child(
            div()
                .class("group-aria-busy:invisible")
                .child_text(&self.label)
        )
    }
}

#[derive(Debug, Default, Clone)]
pub enum Color {
    #[default]
    Primary,
    Gray,
}

impl Color {
    pub fn to_class(self) -> String {
        match self {
            Color::Gray => "bg-neutral-800".to_string(),
            Color::Primary => "bg-blue-600".to_string(),
        }
    }
}
