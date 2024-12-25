use crate::core::html::{div, frag, span, Elem};

use super::icon;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpinnerBlock {
    label: Option<String>,
}

impl SpinnerBlock {
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_owned());
        self
    }

    pub fn view(self) -> Elem {
        div()
            .class("w-full h-full flex flex-col items-center justify-center gap-1.5")
            .child(icon::solid::spinner("size-16 animate-spin"))
            .child(match self.label {
                Some(label) => span().class("text-xs text-muted").child_text(&label),
                None => frag(),
            })
    }
}
