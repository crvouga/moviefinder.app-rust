use crate::core::html::*;

#[derive(Default)]
pub struct BottomButtons {}

impl BottomButtons {
    pub fn view(self) -> Elem {
        div().class("flex items-center justify-center w-full border-t h-20")
    }
}

#[derive(Default)]
pub struct BottomButton {
    text: String,
    icon: Option<Elem>,
    active: bool,
}

impl BottomButton {
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
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
                    "flex flex-1 items-center justify-center gap-0.5 flex-col text-sm h-full cursor-pointer select-none active:opacity-75 transition-opacity",
                    if self.active {
                        "text-blue-500"
                    } else {
                        "text-white"
                    },
                ]
        ).child(
            self.icon.clone().unwrap_or_else(|| frag())
        ).child_text(
            &self.text,
        )
    }
}
