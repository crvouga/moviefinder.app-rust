use crate::core::html::*;

impl Elem {
    pub fn src_swiper(self) -> Self {
        self.src("https://cdn.jsdelivr.net/npm/swiper@11/swiper-element-bundle.min.js")
    }
}

pub fn container() -> Elem {
    elem("swiper-container")
        .attr("css-mode", "true")
        .attr("slides-per-view", "1")
        .attr("direction", "vertical")
        .style("scroll-snap-stop: always;")
}

pub fn slide() -> Elem {
    elem("swiper-slide")
}
