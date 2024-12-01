use std::time::Duration;

use crate::core::{
    datastar::datastar::Builder,
    html::*,
    ui::icon::{self, spinner},
};

#[derive(Debug, Clone, Default)]
pub struct SearchBar {
    search_url: String,
    input_id: String,
    input_search_name: String,
    input_model: String,
    placeholder: String,
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

    pub fn placeholder(mut self, value: &str) -> Self {
        self.placeholder = value.to_string();
        self
    }

    pub fn view(&self) -> Elem {
        label()
            .class("w-full h-20 shrink-0 border-b group flex items-center gap-2 overflow-hidden px-5")
            .data_bind("aria-busy", "$signalFetching")
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
                    .name(&self.input_search_name)
                    .data_on(|b| {
                        b.e("clear")
                            .js("evt.target.value = ''")
                            .js("evt?.target?.focus()")
                            .js("evt?.target?.dispatchEvent(new Event('input'))")
                    })
                    .data_on(|b| {
                        b.input()
                            .debounce(Duration::from_millis(300))
                            .get(&self.search_url)
                    })
                    .data_indicator("signalFetching")
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
                    .data_on(|b| b
                        .click()
                        .js("evt?.target?.parentNode?.querySelector?.('input')?.dispatchEvent(new Event('clear'))")
                    )
                    .aria_label("clear search")
                    .class("h-full place-items-center")
                    .class("grid peer-placeholder-shown:hidden")
                    .child(icon::x_mark("size-8 pointer-events-none")),
            )
    }
}
