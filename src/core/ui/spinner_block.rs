use crate::core::html::{div, Elem};

use super::icon;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpinnerBlock;

impl SpinnerBlock {
    pub fn view(self) -> Elem {
        div()
            .class("w-full h-full flex items-center justify-center")
            .child(icon::solid::spinner("size-16 animate-spin"))
    }
}
