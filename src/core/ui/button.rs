use super::icon;
use crate::core::html::*;

#[derive(Debug, Default)]
pub struct Button {
    label: String,
    color: Color,
    indicator: Option<String>,
}

impl Button {
    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn color_primary(mut self) -> Self {
        self.color = Color::Primary;
        self
    }

    pub fn indicator(mut self, value: &str) -> Self {
        self.indicator = Some(value.to_string());
        self
    }

    pub fn view(self) -> Elem {
        button()
        .class("group relative flex items-center justify-center gap-2 rounded px-4 py-3 text-lg font-bold text-white")
        .class("enabled:hover:opacity-80 enabled:active:opacity-60")
        .class("disabled:opacity-80 disabled:cursor-not-allowed")
        .class(&self.color.to_class())
        .map(|e| {
            if let Some(indicator) = self.indicator {
            e.data_indicator(&indicator).data_bind("aria-busy", &format!("${}", indicator))
            } else {
                e
            }
        })
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
