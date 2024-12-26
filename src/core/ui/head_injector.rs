#![allow(dead_code)]
use crate::core::html::*;

#[derive(Default)]
pub struct HeadInjector {}

impl HeadInjector {
    pub fn view(self) -> Html {
        elem("head-injector")
    }
}

impl Html {
    pub fn src_head_injector(self) -> Self {
        self
        // self.src("./head-injector-element.js")
    }
}
