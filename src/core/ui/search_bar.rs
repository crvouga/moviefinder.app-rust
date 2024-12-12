use std::time::Duration;

use crate::core::{
    datastar::datastar::js_dot_value,
    html::*,
    ui::icon::{self, spinner},
};

#[derive(Default)]
pub struct SearchBar {
    url: String,
    indicator: String,
    input: Option<Box<dyn Fn(Elem) -> Elem>>,
}

impl SearchBar {
    pub fn url(mut self, value: &str) -> Self {
        self.url = value.to_string();
        self
    }

    pub fn indicator(mut self, value: &str) -> Self {
        self.indicator = value.to_string();
        self
    }

    pub fn input(mut self, input: impl Fn(Elem) -> Elem + 'static) -> Self {
        self.input = Some(Box::new(input));
        self
    }

    pub fn view(self) -> Elem {
        let map_input = self.input.unwrap_or_else(|| Box::new(|x| x));
        label()
            .class(
                "w-full h-20 shrink-0 border-b group flex items-center gap-2 overflow-hidden px-5",
            )
            .data_attributes("aria-busy", &js_dot_value(&self.indicator))
            .child(
                div()
                    .class("h-full grid place-items-center pr-2")
                    .child(icon::magnifying_glass("size-8")),
            )
            .child(
                input()
                    .class("flex-1 h-full bg-transparent peer outline-none")
                    .type_("text")
                    .data_indicator(&self.indicator)
                    .data_on(|b| {
                        b.e("clear")
                            .js("evt.target.focus()")
                            .js("evt.target.value = ''")
                            .sse(&self.url)
                    })
                    .data_on(|b| {
                        b.input()
                            .debounce(Duration::from_millis(300))
                            .js("console.log('hello')")
                            .sse(&self.url)
                    })
                    .map(map_input),
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
