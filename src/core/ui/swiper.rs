use crate::core::html::*;

impl Elem {
    pub fn src_swiper(self) -> Self {
        self.src("https://cdn.jsdelivr.net/npm/swiper@11/swiper-element-bundle.min.js")
    }

    pub fn swiper_slides_per_view(self, slides_per_view: &str) -> Self {
        self.attr("slides-per-view", slides_per_view)
    }

    pub fn swiper_direction(self, direction: Direction) -> Self {
        self.attr("direction", direction.as_str())
    }

    pub fn swiper_direction_vertical(self) -> Self {
        self.swiper_direction(Direction::Vertical)
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Vertical,
}

impl Direction {
    pub fn as_str(&self) -> &str {
        match self {
            Direction::Vertical => "vertical",
        }
    }
}

pub fn container() -> Elem {
    elem("swiper-container")
}

pub fn slide() -> Elem {
    elem("swiper-slide")
}
