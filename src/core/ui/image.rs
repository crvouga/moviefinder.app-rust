use crate::core::html::*;

pub struct Image {}

impl Image {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(self) -> Elem {
        elem("image-element")
    }
}

impl Elem {
    pub fn src_image_element(self) -> Self {
        self.src("./image-element.js")
    }
}
