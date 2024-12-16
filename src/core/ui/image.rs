use crate::core::html::*;

pub struct Image {}

impl Image {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(self) -> Elem {
        img()
            .class(
                "w-full h-full object-cover bg-neutral-700 animate-pulse border-none outline-none",
            )
            .on_load("this.classList.remove('animate-pulse')")
    }
}

impl Elem {
    pub fn src_image_element(self) -> Self {
        self
        // self.src("./image-element.js")
    }
}
