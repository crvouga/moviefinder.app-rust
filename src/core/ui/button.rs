use super::icon;
use crate::core::{html::*, js::Js};

#[derive(Debug, Default)]
pub struct Button {
    label: String,
    color: ButtonColor,
    indicator: Option<String>,
    id: Option<String>,
}

impl Button {
    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn color(mut self, color: ButtonColor) -> Self {
        self.color = color;
        self
    }

    pub fn color_gray(self) -> Self {
        self.color(ButtonColor::Gray)
    }

    pub fn color_primary(self) -> Self {
        self.color(ButtonColor::Primary)
    }

    pub fn indicator(mut self, value: &str) -> Self {
        self.indicator = Some(value.to_string());
        self
    }

    pub fn view(self) -> Elem {
        let signal_indicator = Js::dot_value(&self.indicator.clone().unwrap_or_default());
        let id = self.id.unwrap_or("".to_owned());

        button()
        .id(&id)
        .class("group relative flex items-center justify-center gap-2 rounded px-4 py-3 text-lg font-bold text-white")
        .class("enabled:hover:opacity-80 enabled:active:opacity-60")
        .class("disabled:opacity-80 disabled:cursor-not-allowed")
        .class("aria-busy:opacity-100 aria-busy:cursor-wait")
        .class(&self.color.to_class())
        .type_("button")
        .map(|e: Elem| {
            if let Some(indicator) = self.indicator {
                e.data_indicator(&indicator).data_attributes("aria-busy", &signal_indicator).data_attributes("disabled", &signal_indicator)
            } else {
                e
            }
        })
        .child(
            div()
                .class("absolute inset-0 flex items-center justify-center opacity-0 group-aria-busy:opacity-100")
                .child(icon::solid::spinner("size-8 animate-spin"))
        )
        .child(
            div()
                .class("group-aria-busy:invisible")
                .child_text(&self.label)
        )
    }
}

#[derive(Debug, Default, Clone)]
pub enum ButtonColor {
    #[default]
    Primary,
    Gray,
}

impl ButtonColor {
    pub fn to_class(self) -> String {
        match self {
            ButtonColor::Gray => "bg-neutral-800".to_string(),
            ButtonColor::Primary => "bg-blue-600".to_string(),
        }
    }
}
