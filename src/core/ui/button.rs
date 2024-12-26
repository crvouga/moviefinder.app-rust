use super::icon;
use crate::core::{html::*, js::Js};

#[derive(Default)]
pub struct Button {
    label: String,
    color: ButtonColor,
    size: ButtonSize,
    indicator: Option<String>,
    id: Option<String>,
    map_button: Option<Box<dyn Fn(Html) -> Html>>,
}

impl Button {
    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn map_button(mut self, map_button: impl Fn(Html) -> Html + 'static) -> Self {
        self.map_button = Some(Box::new(map_button));
        self
    }

    pub fn color(mut self, color: ButtonColor) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn size_small(self) -> Self {
        self.size(ButtonSize::Small)
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

    pub fn view(self) -> Html {
        let signal_indicator = Js::dot_value(&self.indicator.clone().unwrap_or_default());
        let id = self.id.unwrap_or("".to_owned());

        let map_button = self.map_button.unwrap_or_else(|| Box::new(|x| x));

        button()
        .id(&id)
        .class("group relative flex items-center justify-center rounded font-bold text-white")
        .class("enabled:hover:opacity-80 enabled:active:opacity-active")
        .class("disabled:opacity-80 disabled:cursor-not-allowed")
        .class("aria-busy:opacity-100 aria-busy:cursor-wait")
        .class(&self.size.to_class())
        .class(&self.color.to_class())
        .map(map_button)
        .type_button()
        .map(|e: Html| {
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

#[derive(Debug, Default, Clone)]
pub enum ButtonSize {
    Small,
    #[default]
    Large,
}

impl ButtonSize {
    pub fn to_class(self) -> String {
        match self {
            ButtonSize::Small => "gap-0.5 px-3 py-3 text-sm".to_string(),
            ButtonSize::Large => "gap-2 px-4 py-3 text-lg".to_string(),
        }
    }
}
