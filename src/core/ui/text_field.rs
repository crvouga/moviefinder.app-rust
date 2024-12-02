use crate::core::{
    datastar::datastar::{js_not, signal},
    html::{button, children::text, div, input, label, span, Elem},
};

use super::icon;

#[derive(Default)]
pub struct TextField {
    label: String,
    placeholder: String,
    input: Option<Box<dyn Fn(Elem) -> Elem>>,
    error: Option<String>,
    model_error: String,
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

    pub fn input(mut self, input: impl Fn(Elem) -> Elem + 'static) -> Self {
        self.input = Some(Box::new(input));
        self
    }

    pub fn model_error(mut self, value: &str) -> Self {
        self.model_error = value.to_string();
        self
    }

    pub fn error(mut self, error: &str) -> Self {
        if error.is_empty() {
            return self;
        }
        self.error = Some(error.to_string());
        self
    }

    fn signal_error(&self) -> String {
        signal(&self.model_error)
    }

    fn signal_has_error(&self) -> String {
        format!("{}.length > 0", self.signal_error())
    }

    pub fn view(self) -> Elem {
        let signal_has_error = self.signal_has_error();
        let signal_error = self.signal_error();
        let map_input = self.input.unwrap_or_else(|| Box::new(|x| x));
        label()
            .class("w-full flex flex-col gap-2")
            .child(
                span()
                    .child(text(&self.label))
                    .class("font-bold")
                    .data_class(|c| c.c("text-red-500", &signal_has_error)),
            )
            .child(
                div()
                    .class("p-4 bg-neutral-900 border-2 rounded w-full flex items-center gap-2")
                    .data_class(|c| {
                        c.c(
                            "border-red-500 focus:border-offset-2 focus:border-red-500",
                            &signal_has_error,
                        )
                        .c(
                            "border-neutral-700 focus:border-offset-2 focus:border-blue-500",
                            &js_not(&signal_has_error),
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
                                b.click()
                                    .js("evt.target.parentNode.querySelector('input').dispatchEvent(new Event('clear'))")
                            })
                            .aria_label("clear search")
                            .class("h-full place-items-center")
                            .class("grid peer-placeholder-shown:hidden")
                            .child(icon::x_mark("size-8 pointer-events-none")),
                    )
            )
            .child(
                span()
                    .class("font-bold text-red-500")
                    .data_text(&signal_error),
            )
    }
}
