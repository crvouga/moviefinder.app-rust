use std::time::Duration;

use crate::core::{html::*, js::Js, ui::icon::solid::spinner};

use super::icon;

#[derive(Default)]
pub struct SearchBar {
    url: String,
    indicator: String,
    input: Option<Box<dyn Fn(Html) -> Html>>,
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

    pub fn input(mut self, input: impl Fn(Html) -> Html + 'static) -> Self {
        self.input = Some(Box::new(input));
        self
    }

    pub fn view(self) -> Html {
        let map_input = self.input.unwrap_or_else(|| Box::new(|x| x));
        label()
            .class(
                "w-full h-bar shrink-0 border-b group flex items-center gap-2 overflow-hidden px-5",
            )
            .data_attributes("aria-busy", &Js::dot_value(&self.indicator))
            .child(
                div()
                    .class("h-full grid place-items-center pr-2")
                    .child(icon::solid::magnifying_glass("size-8")),
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
                            .js("evt.target.dispatchEvent(new Event('input'))")

                    })
                    .data_on(|b| {
                        b.input()
                            .debounce(Duration::from_millis(300))
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
                    .type_button()
                    .tab_index(0)
                    .data_on(|b| {
                        b.press_down()
                            .js("evt.target.parentNode.querySelector('input').dispatchEvent(new Event('clear'))")
                    })
                    .aria_label("clear search")
                    .class("h-full place-items-center")
                    .class("grid peer-placeholder-shown:hidden")
                    .child(icon::solid::x_mark("size-8 pointer-events-none")),
            )
    }
}
