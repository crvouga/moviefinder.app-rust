use std::time::Duration;

use crate::core::{
    datastar::datastar::dollar,
    html::*,
    ui::icon::{self, spinner},
};

#[derive(Debug, Clone, Default)]
pub struct SearchBar {
    url: String,
    input_id: String,
    input_model: String,
    placeholder: String,
    indicator: String,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            placeholder: "Search".to_string(),
            ..Self::default()
        }
    }

    pub fn input_model(mut self, value: &str) -> Self {
        self.input_model = value.to_string();
        self
    }

    pub fn url(mut self, value: &str) -> Self {
        self.url = value.to_string();
        self
    }

    pub fn input_id(mut self, value: &str) -> Self {
        self.input_id = value.to_string();
        self
    }

    pub fn indicator(mut self, value: &str) -> Self {
        self.indicator = value.to_string();
        self
    }

    pub fn placeholder(mut self, value: &str) -> Self {
        self.placeholder = value.to_string();
        self
    }

    pub fn view(&self) -> Elem {
        label()
            .class(
                "w-full h-20 shrink-0 border-b group flex items-center gap-2 overflow-hidden px-5",
            )
            .data_bind("aria-busy", &dollar(&self.indicator))
            .child(
                div()
                    .class("h-full grid place-items-center pr-2")
                    .child(icon::magnifying_glass("size-8")),
            )
            .child(
                input()
                    .id(&self.input_id)
                    .class("flex-1 h-full bg-transparent peer outline-none")
                    .data_model(&self.input_model)
                    .type_("text")
                    .data_indicator(&self.indicator)
                    .data_on(|b| {
                        b.e("clear")
                            .js("evt.target.focus()")
                            .js(&format!("${} = ''", &self.input_model))
                            .post(&self.url)
                    })
                    .data_on(|b| {
                        b.input()
                            .debounce(Duration::from_millis(300))
                            .post(&self.url)
                    })
                    .placeholder(&self.placeholder),
            )
            .child(
                div()
                    .class("group-aria-busy:block hidden")
                    .child(spinner("size-8 animate-spin")),
            )
            .child(
                button()
                    .type_("button")
                    .tab_index(0)
                    .data_on(|b| {
                        b.click()
                            .js("evt.target.parentNode.querySelector('input').dispatchEvent(new Event('clear'))")
                            
                    })
                    .aria_label("clear search")
                    .class("h-full place-items-center")
                    .class("grid peer-placeholder-shown:hidden")
                    .child(icon::x_mark("size-8 pointer-events-none")),
            )
    }
}
