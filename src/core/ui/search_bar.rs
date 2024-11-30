use std::time::Duration;

use crate::core::{
    html::*,
    ui::icon::{self, spinner},
};

#[derive(Debug, Clone, Default)]
pub struct SearchBar {
    search_url: String,
    input_id: String,
    input_search_name: String,
    input_model: String,
}

impl SearchBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_model(mut self, value: &str) -> Self {
        self.input_model = value.to_string();
        self
    }

    pub fn search_url(mut self, value: &str) -> Self {
        self.search_url = value.to_string();
        self
    }

    pub fn input_id(mut self, value: &str) -> Self {
        self.input_id = value.to_string();
        self
    }

    pub fn input_name(mut self, input_search_name: &str) -> Self {
        self.input_search_name = input_search_name.to_string();
        self
    }

    pub fn view(&self) -> Elem {
        label()
            // .data(|a| a.on().input().debounce(500).js_get(&self.search_url).js("console.log('hello')"))
            .class("w-full h-16 shrink-0 border-b group flex items-center gap-2 overflow-hidden")
            .data_bind("aria-busy", "$signalFetching")
            .child(
                div()
                    .class("h-full grid place-items-center pl-4 pr-2")
                    .child(icon::magnifying_glass("size-6")),
            )
            .child(
                input()
                    .id(&self.input_id)
                    .class("flex-1 h-full bg-transparent peer outline-none")
                    .data_model(&self.input_model)
                    .type_("text")
                    .name(&self.input_search_name)
                    .data_on("clear", "evt.target.value = ''")
                    .data_on("clear", "evt?.target?.focus()")
                    .data_on_input_debounce_get(Duration::from_millis(250), &self.search_url)
                    .data_indicator("signalFetching")
                    .placeholder("Search"),
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
                    // .data_on_click("evt?.target?.parentNode?.querySelector?.('input')?.dispatchEvent(new Event('clear'))")
                    .data_on_click(&format!("${} = ''", self.input_model))
                    .aria_label("clear search")
                    .class("h-full pr-5 place-items-center")
                    .class("grid peer-placeholder-shown:hidden")
                    .child(icon::x_circle_mark("size-6 pointer-events-none")),
            )
    }
}
