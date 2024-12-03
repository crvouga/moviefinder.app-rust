use crate::core::html::*;

use super::head_injector::HeadInjector;

pub struct Image {}

impl Image {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(self) -> Elem {
        elem("image-element").child(
            HeadInjector::default()
                .view()
                .child(script().src_image_element()),
        )
    }
}

impl Elem {
    pub fn src_image_element(self) -> Self {
        self.src("./image-element.js")
    }
}
