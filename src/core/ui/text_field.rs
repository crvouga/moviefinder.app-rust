use crate::core::{
    html::{button, children::text, div, input, label, span, Elem},
    js::Js,
};

use super::icon;

#[derive(Default)]
pub struct TextField {
    label: String,
    placeholder: String,
    map_input: Option<Box<dyn Fn(Elem) -> Elem>>,
    bind_error: Option<String>,
}

impl TextField {
    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn map_input(mut self, map_input: impl Fn(Elem) -> Elem + 'static) -> Self {
        self.map_input = Some(Box::new(map_input));
        self
    }

    pub fn bind_error(mut self, value: &str) -> Self {
        self.bind_error = Some(value.to_string());
        self
    }

    fn signal_error(&self) -> Option<String> {
        self.bind_error.clone().map(|x| Js::dot_value(&x))
    }

    fn signal_has_error(&self) -> Option<String> {
        self.signal_error().map(|x| format!("{}?.length > 0", x))
    }

    pub fn view(self) -> Elem {
        let signal_has_error = self.signal_has_error();
        let signal_error = self.signal_error();
        let map_input = self.map_input.unwrap_or_else(|| Box::new(|x| x));
        label()
            .class("w-full flex flex-col gap-2")
            .child(
                span()
                    .child(text(&self.label))
                    .class("font-bold")
                    .data_class(|c| c.maybe_class("text-red-500", &signal_has_error)),
            )
            .child(
                div()
                    .class("p-4 bg-neutral-900 border-2 rounded w-full flex items-center gap-2 min-w-0 overflow-hidden")
                    .data_class(|c| {
                        c.maybe_class(
                            "border-red-500 focus:border-offset-2 focus:border-red-500",
                            &signal_has_error,
                        )
                        .maybe_class(
                            "border-neutral-700 focus:border-offset-2 focus:border-blue-500",
                            &signal_has_error.map(|s| Js::not(&s)),
                        )
                    })
                    .child(
                        input()
                            .class("min-h-8 text-lg flex-1 h-full peer bg-transparent outline-none")
                            .type_("text")
                            .placeholder(&self.placeholder)
                            .data_on(|b| b.e("clear")
                                .js("evt.target.focus()")
                                .js("evt.target.value = ''")
                                .js("evt.target.dispatchEvent(new Event('input'))"))
                            .map(map_input),
                    )
                    .child(
                        button()
                            .type_("button")
                            .tab_index(0)
                            .data_on(|b| {
                                b.press_down()
                                    .js("evt.target.parentNode.querySelector('input').dispatchEvent(new Event('clear'))")
                            })
                            .aria_label("clear search")
                            .class("h-full place-items-center shrink-0 w-fit")
                            .class("grid peer-placeholder-shown:hidden")
                            .child(icon::solid::x_mark("size-8 pointer-events-none")),
                    )
            )
            .map(|e| {
                match signal_error {
                    Some(signal_error) => e.child(
                        span()
                            .class("font-bold text-red-500")
                            .data_text(&signal_error),
                    ),
                    None => e,
                }
            })
    }
}
