use crate::core::html::*;
use crate::core::{
    datastar::datastar::js_dot_value,
    html::{div, frag, Elem},
};

#[derive(Default)]
pub struct Drawer {
    model_open: String,
    initial_open: bool,
    content: Option<Elem>,
    on_close: String,
}

impl Elem {
    pub fn src_drawer_element(self) -> Self {
        self.src("./drawer-element.js")
    }
}

impl Drawer {
    pub fn model_open(mut self, value: &str) -> Self {
        self.model_open = value.to_string();
        self
    }

    pub fn initial_open(mut self, value: bool) -> Self {
        self.initial_open = value;
        self
    }

    pub fn on_close(mut self, value: &str) -> Self {
        self.on_close = value.to_string();
        self
    }

    pub fn content(mut self, value: Elem) -> Self {
        self.content = Some(value);
        self
    }

    pub fn view_root() -> Elem {
        div().id("drawer-root")
    }

    pub fn view(self) -> Elem {
        let initial_open = if self.initial_open { "true" } else { "false" };
        let store_entry_open = if self.model_open.is_empty() {
            ""
        } else {
            &format!("{}: {}", self.model_open, initial_open)
        };
        let store = format!("{{ {}, isLoaded: true }}", store_entry_open);

        Self::view_root().data_signals(&store).child(
            elem("drawer-element")
                .data_show("isLoaded.value")
                .data_attributes("open", &js_dot_value(&self.model_open))
                .attr("open", initial_open)
                .data_on(|b| b.e("close").js(&self.on_close))
                .child(
                    div()
                        .class(
                            "h-fit max-h-full w-full overflow-hidden border bg-black rounded-t-2xl",
                        )
                        .child(self.content.unwrap_or(frag())),
                ),
        )
    }
}
