use children::text;

use crate::core::html::*;
use crate::core::html::{div, frag, Html};
use crate::core::js::Js;

#[derive(Default)]
pub struct Drawer {
    model_open: String,
    initial_open: bool,
    content: Option<Html>,
    on_close: String,
}

impl Html {
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

    pub fn content(mut self, value: Html) -> Self {
        self.content = Some(value);
        self
    }

    pub fn view_root() -> Html {
        div().id("drawer-root")
    }

    pub fn view(self) -> Html {
        let initial_open = if self.initial_open { "true" } else { "false" };

        Self::view_root()
        .data_signal("is_loaded", "true")
        .map(|e| {
            if self.model_open.is_empty() {
                e
            } else {
                e.data_signal(&self.model_open, initial_open)
            }
        })
        .child(
            elem("drawer-element")
                .data_show("is_loaded.value")
                .data_attributes("open", &Js::dot_value(&self.model_open))
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

#[derive(Default)]
pub struct DrawerTitle {
    title: String,
}

impl DrawerTitle {
    pub fn title(mut self, value: &str) -> Self {
        self.title = value.to_string();
        self
    }

    pub fn view(self) -> Html {
        div()
            .class("text-3xl font-bold text-left w-full px-6 pt-6 pb-4")
            .child(text(&self.title))
    }
}

#[derive(Default)]
pub struct DrawerBody {
    content: String,
}

impl DrawerBody {
    pub fn content(mut self, value: &str) -> Self {
        self.content = value.to_string();
        self
    }

    pub fn view(self) -> Html {
        div()
            .class("text-base text-secondary font-normal text-left w-full px-6 pb-6")
            .child(text(&self.content))
    }
}
