#![allow(dead_code)]
use crate::core::html::*;

#[derive(Default)]
pub struct HeadInjector {}

impl HeadInjector {
    pub fn view(self) -> Elem {
        elem("head-injector")
    }
}

impl Elem {
    pub fn src_head_injector(self) -> Self {
        self.src("./head-injector-element.js")
    }
}
