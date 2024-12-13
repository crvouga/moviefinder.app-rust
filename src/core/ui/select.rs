use crate::core::{
    datastar::datastar::{js_dot_value, js_not},
    html::{children::text, div, label, option, select, span, Elem},
};

use super::icon;

#[derive(Default)]
pub struct Select {
    select_id: String,
    label: String,
    placeholder: String,
    options: Vec<SelectOption>,
    bind_error: String,
    map_select: Option<Box<dyn Fn(Elem) -> Elem>>,
}

impl Select {
    pub fn select_id(mut self, select_id: &str) -> Self {
        self.select_id = select_id.to_string();
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    pub fn map_select(mut self, map_select: impl Fn(Elem) -> Elem + 'static) -> Self {
        self.map_select = Some(Box::new(map_select));
        self
    }

    pub fn bind_error(mut self, value: &str) -> Self {
        self.bind_error = value.to_string();
        self
    }

    fn signal_error(&self) -> String {
        js_dot_value(&self.bind_error)
    }

    fn signal_has_error(&self) -> String {
        format!("{}?.length > 0", self.signal_error())
    }

    pub fn view(self) -> Elem {
        let signal_has_error = self.signal_has_error();
        let signal_error = self.signal_error();
        let map_select = self.map_select.unwrap_or_else(|| Box::new(|x| x));

        label()
            .class("w-full flex flex-col gap-2")
            .for_(self.select_id.as_str())
            .data_on(|e| e.click().js("const s = evt.target.querySelector('select'); s?.focus()"))
            .child(
                span()
                    .child(text(&self.label))
                    .class("font-bold")
                    .data_class(|c| c.c("text-red-500", &signal_has_error)),
            )
            .child(
                div()
                    .class("p-4 bg-neutral-900 border-2 rounded w-full flex flex-row items-center gap-2 min-w-0 overflow-hidden max-w-full")
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
                        div().class("flex-1 flex items-center overflow-hidden").child(
                            select()
                                .id(&self.select_id)
                                .class("min-h-8 text-lg w-full h-full bg-transparent outline-none appearance-none")
                                .map(map_select)
                                .children(
                                    self.options.into_iter().map(|o| o.view()).collect(),
                                )
                        )
                    )
                    .child(
                        div()
                            .class("grid h-full place-items-center shrink-0 w-fit")
                            .child(icon::solid::chevron_down("size-8 pointer-events-none")),
                    )
            )
            .child(
                span()
                    .class("font-bold text-red-500")
                    .data_text(&signal_error),
            )
    }
}

#[derive(Default)]
pub struct SelectOption {
    value: String,
    label: String,
}

impl SelectOption {
    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn view(self) -> Elem {
        option().value(&self.value).child(text(&self.label))
    }
}
