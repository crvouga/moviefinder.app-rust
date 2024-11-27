use crate::core::html::*;

pub struct HeadInjector {}

impl HeadInjector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(self) -> Elem {
        elem("head-injector")
    }
}

impl Elem {
    pub fn src_head_injector(self) -> Self {
        self.src("./head-injector-element.js")
    }
}
