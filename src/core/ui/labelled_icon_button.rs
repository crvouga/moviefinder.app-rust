use crate::core::html::{button, div, frag, span, Html};

#[derive(Default)]
pub struct LabelledIconButton {
    text: String,
    icon: Option<Html>,
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

    pub fn icon(mut self, icon: Html) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn view(&self) -> Html {
        button()
        .class("flex flex-col gap-1 items-center justify-center")
        .class("active:opacity-active")
        .class(
            if self.disabled {
                "opacity-30 pointer-events-none cursor-not-allowed"
            } else {
                ""
            },
        )
        .id(&self.id)
        .child(
            div()
            .class("flex flex-1 items-center justify-center gap-0.5 flex-col h-full cursor-pointer select-none p-2 overflow-hidden bg-black rounded-full aspect-square")
            .class(
                if self.active && !self.disabled {
                    "text-blue-500"
                } else {
                    ""
                },
            )
            .disabled(self.disabled)
            .child(self.icon.clone().unwrap_or_else(|| frag()))
        ).child(
            span().class("bg-black font-bold rounded text-xs w-fit px-1 py-0.5").child_text(&self.text)
        )
    }
}
