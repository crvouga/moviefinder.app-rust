use crate::core::html::*;

impl Elem {
    pub fn src_swiper_cdn(self) -> Self {
        self.src("https://cdn.jsdelivr.net/npm/swiper@11/swiper-element-bundle.min.js")
        // self.src("./swiper-element.js")
    }
}

pub fn container() -> Elem {
    elem("swiper-container")
        .attr("css-mode", "false")
        .attr("slides-per-view", "1")
        .attr("direction", "vertical")
}

pub fn slide() -> Elem {
    elem("swiper-slide")
}
