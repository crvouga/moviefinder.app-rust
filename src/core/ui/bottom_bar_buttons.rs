use crate::core::html::*;

#[derive(Default)]
pub struct BottomButtons {
    bottom_buttons: Vec<BottomButton>,
}

impl BottomButtons {
    pub fn new() -> Self {
        BottomButtons::default()
    }

    pub fn add(mut self, bottom_button: BottomButton) -> Self {
        self.bottom_buttons.push(bottom_button);
        self
    }

    pub fn view(self) -> Elem {
        div_()
            .class("flex items-center justify-center w-full border-t divide-x h-16")
            .children(
                &self
                    .bottom_buttons
                    .iter()
                    .map(|btn| btn.view())
                    .collect::<Vec<Elem>>(),
            )
    }
}

#[derive(Default)]
pub struct BottomButton {
    text: String,
    hx_get: String,
    hx_target: String,
    icon: Option<Elem>,
    active: bool,
}

impl BottomButton {
    pub fn new() -> Self {
        BottomButton::default()
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn hx_get(mut self, hx_get: &str) -> Self {
        self.hx_get = hx_get.to_string();
        self
    }

    pub fn hx_target(mut self, hx_target: &str) -> Self {
        self.hx_target = hx_target.to_string();
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
        a_()
        .hx_get(&self.hx_get)
        .hx_target(&self.hx_target)
        .hx_swap_inner_html()
        .hx_push_url()
        .hx_trigger_click()
        .hx_preload_mouse_down()
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
            self.icon.clone().unwrap_or_else(|| fragment_())
        ).child_text(
            &self.text,
        )
    }
}
