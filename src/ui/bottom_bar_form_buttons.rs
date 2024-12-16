use crate::core::{
    datastar::datastar::DataOn,
    html::{div, Elem},
    ui::button::Button,
};

pub const SIGNAL_IS_SUBMITTING: &str = "signal_is_submitting";

#[derive(Default)]
pub struct BottomBarFormButtons {
    on_cancel: Option<Box<dyn FnOnce(DataOn) -> DataOn>>,
    submit_indicator: Option<String>,
    submit_label: Option<String>,
}

impl BottomBarFormButtons {
    pub fn on_cancel(mut self, on_cancel: impl FnOnce(DataOn) -> DataOn + 'static) -> Self {
        self.on_cancel = Some(Box::new(on_cancel));
        self
    }

    pub fn submit_indicator(mut self, submit_indicator: &str) -> Self {
        self.submit_indicator = Some(submit_indicator.to_string());
        self
    }

    pub fn submit_label(mut self, submit_label: &str) -> Self {
        self.submit_label = Some(submit_label.to_string());
        self
    }

    pub fn view(self) -> Elem {
        let on_cancel = self.on_cancel.unwrap_or_else(|| Box::new(|d| d));
        let submit_label = self.submit_label.unwrap_or_else(|| "Submit".to_string());

        div()
            .id("bottom-bar-form")
            .data_signal(SIGNAL_IS_SUBMITTING, "false")
            .class(
                "flex-none flex flex-row items-center justify-center p-4 border-t gap-4 min-h-bar w-full",
            )
            .child(
                Button::default()
                    .label("Cancel")
                    .color_gray()
                    .view()
                    .data_on(on_cancel)
                    .type_("button")
                    .id("bottom-bar-form-cancel-button")
                    .class("flex-1"),
            )
            .child(
                Button::default()
                    .label(&submit_label)
                    .color_primary()
                    .indicator(SIGNAL_IS_SUBMITTING)
                    .view()
                    .id("bottom-bar-form-submit-button")
                    .map(|e| match self.submit_indicator {
                        Some(ref indicator) => e.data_indicator(&indicator),
                        None => e,
                    })
                    .type_("submit")
                    .class("flex-1"),
            )
    }
}
