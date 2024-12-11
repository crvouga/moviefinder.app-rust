use crate::core::{
    html::{div, Elem},
    ui::button::{Button, ButtonColor},
};

const SIGNAL_IS_SAVING: &str = "signal_is_saving";

#[derive(Debug, Default)]
pub struct BottomBarForm {
    cancel_url: String,
    save_url: String,
}

impl BottomBarForm {
    pub fn cancel_url(mut self, cancel_url: &str) -> Self {
        self.cancel_url = cancel_url.to_string();
        self
    }

    pub fn save_url(mut self, save_url: &str) -> Self {
        self.save_url = save_url.to_string();
        self
    }

    pub fn view(&self) -> Elem {
        div()
            .id("bottom-bar-form")
            .class(
                "flex-none flex flex-row items-center justify-center p-4 border-t gap-4 min-h-20",
            )
            .child(
                Button::default()
                    .label("Cancel")
                    .color(ButtonColor::Gray)
                    .view()
                    .data_on(|b| b.click().push_then_get(&self.cancel_url))
                    .type_("button")
                    .class("flex-1"),
            )
            .child(
                Button::default()
                    .label("Save")
                    .color(ButtonColor::Primary)
                    .indicator(SIGNAL_IS_SAVING)
                    .view()
                    .data_on(|b| b.click().sse(&self.save_url))
                    .id("save-button")
                    .class("flex-1"),
            )
    }
}
