use crate::core::{
    html::*,
    ui::icon::{self, spinner},
};

#[derive(Debug, Clone, Default)]
pub struct SearchBar {
    inputted_search_path: String,
    inputted_search_target: String,
    input_search_id: String,
    input_search_name: String,
}

impl SearchBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inputted_search_path(mut self, inputted_search_path: &str) -> Self {
        self.inputted_search_path = inputted_search_path.to_string();
        self
    }

    pub fn inputted_search_target(mut self, inputted_search_target: &str) -> Self {
        self.inputted_search_target = inputted_search_target.to_string();
        self
    }

    pub fn input_search_id(mut self, input_search_id: &str) -> Self {
        self.input_search_id = input_search_id.to_string();
        self
    }

    pub fn input_search_name(mut self, input_search_name: &str) -> Self {
        self.input_search_name = input_search_name.to_string();
        self
    }

    pub fn view(&self) -> Elem {
        label()
            .class("w-full h-16 shrink-0 border-b group flex items-center gap-2 overflow-hidden")
            .child(
                div()
                    .class("h-full grid place-items-center pl-4 pr-2")
                    .child(icon::magnifying_glass("size-6")),
            )
            .child(
                input()
                    .id(&self.input_search_id)
                    .class("flex-1 h-full bg-transparent peer outline-none")
                    .type_("text")
                    .name(&self.input_search_name)
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
                    .aria_label("clear search")
                    .class("h-full pr-5 place-items-center")
                    .class("grid peer-placeholder-shown:hidden")
                    .child(icon::x_circle_mark("size-6")),
            )
    }
}
