use crate::core::html::{children::text, input, label, span, Elem};

#[derive(Default)]
pub struct TextField {
    label: String,
    placeholder: String,
    input: Option<Box<dyn Fn(Elem) -> Elem>>,
    error: Option<String>,
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

    pub fn error(mut self, error: &str) -> Self {
        if error.is_empty() {
            return self;
        }
        self.error = Some(error.to_string());
        self
    }

    pub fn signal_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }

    pub fn view(self) -> Elem {
        let map_input = self.input.unwrap_or_else(|| Box::new(|x| x));

        label()
            .class("w-full flex flex-col gap-2")
            .child(span().child(text(&self.label)).class("font-bold").class(
                if self.error.is_some() {
                    "text-red-500"
                } else {
                    ""
                },
            ))
            .child(
                input()
                    .class("text-lg w-full p-4 bg-neutral-900 border-2 rounded")
                    .class(if self.error.is_some() {
                        "border-red-500 outline-offset-2 outline-red-500"
                    } else {
                        "border-neutral-700 outline-offset-2 outline-blue-500"
                    })
                    .type_("text")
                    .placeholder(&self.placeholder)
                    .map(map_input),
            )
            .map(|e| match self.error {
                Some(error) => e.child(span().class("font-bold text-red-500").child(text(&error))),
                None => e,
            })
    }
}
