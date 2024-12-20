use crate::core::html::{button, frag, Elem};

#[derive(Default)]
pub struct LabelledIconButton {
    text: String,
    icon: Option<Elem>,
    active: bool,
    disabled: bool,
    id: String,
}

impl LabelledIconButton {
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon(mut self, icon: Elem) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn view(&self) -> Elem {
        button()
        .class_list(
            &[
                    "flex flex-1 items-center justify-center gap-0.5 flex-col text-sm h-full cursor-pointer select-none active:opacity-75 size-20 overflow-hidden bg-black/50 rounded-full aspect-square",
                    if self.active && !self.disabled {
                        "text-blue-500"
                    } else {
                        ""
                    },
                    if self.disabled {
                        "opacity-30 pointer-events-none cursor-not-allowed"
                    } else {
                        ""
                    },
                ]
        )
        .disabled(self.disabled)
        .child(self.icon.clone().unwrap_or_else(|| frag())
        ).child_text(&self.text)
        .id(&self.id)
    }
}
