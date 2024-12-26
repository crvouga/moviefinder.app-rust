use crate::core::html::*;

impl Html {
    pub fn src_swiper_cdn(self) -> Self {
        self.src("https://cdn.jsdelivr.net/npm/swiper@11/swiper-element-bundle.min.js")
        // self.src("./swiper-element.js")
    }
}

pub fn container() -> Html {
    elem("swiper-container")
        .attr("css-mode", "false")
        .attr("slides-per-view", "1")
        .attr("direction", "vertical")
}

pub fn slide() -> Html {
    elem("swiper-slide")
}
