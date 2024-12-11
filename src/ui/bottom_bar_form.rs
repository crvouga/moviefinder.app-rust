use crate::core::{
    html::{div, Elem},
    ui::button::Button,
};

pub const SIGNAL_IS_SUBMITTING: &str = "signal_is_submitting";

#[derive(Debug, Default)]
pub struct BottomBarForm {
    cancel_url: String,
    save_url: Option<String>,
}

impl BottomBarForm {
    pub fn cancel_url(mut self, cancel_url: &str) -> Self {
        self.cancel_url = cancel_url.to_string();
        self
    }

    pub fn save_url(mut self, save_url: &str) -> Self {
        self.save_url = Some(save_url.to_string());
        self
    }

    pub fn view(&self) -> Elem {
        div()
            .id("bottom-bar-form")
            .data_signal(SIGNAL_IS_SUBMITTING, "false")
            .class(
                "flex-none flex flex-row items-center justify-center p-4 border-t gap-4 min-h-20",
            )
            .child(
                Button::default()
                    .label("Cancel")
                    .color_gray()
                    .view()
                    .data_on(|b| b.click().push_then_sse(&self.cancel_url))
                    .type_("button")
                    .class("flex-1"),
            )
            .child(
                Button::default()
                    .label("Save")
                    .color_primary()
                    .indicator(SIGNAL_IS_SUBMITTING)
                    .view()
                    .data_on(|b| match &self.save_url {
                        Some(save_url) => b.click().sse(save_url),
                        None => b,
                    })
                    .id("save-button")
                    .type_("submit")
                    .class("flex-1"),
            )
    }
}
