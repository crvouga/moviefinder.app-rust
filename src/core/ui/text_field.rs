use crate::core::{
    datastar::datastar::{js_not, signal},
    html::{children::text, input, label, span, Elem},
};

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
                input()
                    .class("text-lg w-full p-4 bg-neutral-900 border-2 rounded")
                    .data_class(|c| {
                        c.c(
                            "border-red-500 outline-offset-2 outline-red-500",
                            &signal_has_error,
                        )
                        .c(
                            "border-neutral-700 outline-offset-2 outline-blue-500",
                            &js_not(&signal_has_error),
                        )
                    })
                    .type_("text")
                    .placeholder(&self.placeholder)
                    .map(map_input),
            )
            .child(
                span()
                    .class("font-bold text-red-500")
                    .data_text(&signal_error),
            )
    }
}
