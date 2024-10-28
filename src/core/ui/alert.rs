use crate::core::html::*;

pub enum AlertVariant {
    Error,
}

pub struct Alert {
    label: String,
    variant: AlertVariant,
}

impl Alert {
    pub fn error() -> Self {
        Self {
            label: "".to_string(),
            variant: AlertVariant::Error,
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn view(self) -> Elem {
        match self.variant {
            AlertVariant::Error => {
                div()
                .class("relative flex w-full items-center justify-start rounded border border-red-400 bg-red-800 px-4 py-3 text-white")
                .child_text(&self.label)
            }
        }
    }
}
