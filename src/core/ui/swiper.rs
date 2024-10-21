use crate::core::html::*;

pub fn slides_per_view(slides_per_view: &str) -> Attr {
    attr("slides-per-view", slides_per_view)
}

pub enum Direction {
    // Horizontal,
    Vertical,
}

impl Into<Attr> for Direction {
    fn into(self) -> Attr {
        match self {
            // Direction::Horizontal => attr("direction", "horizontal"),
            Direction::Vertical => attr("direction", "vertical"),
        }
    }
}

pub fn container(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("swiper-container", attrs, children)
}

pub fn slide(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("swiper-slide", attrs, children)
}
