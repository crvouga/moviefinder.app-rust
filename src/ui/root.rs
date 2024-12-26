use super::screen::Screen;
use crate::core::{
    html::*,
    js::Js,
    tmdb_api::TMDB_IMAGE_BASE_URL,
    ui::{drawer::Drawer, toast::Toast},
};

pub struct Root {
    url: String,
}

impl Root {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn view(self) -> Elem {
        html()
        .lang("en")
        .style("background-color: #000")
        .child(
            head().children(vec![
                meta().charset("UTF-8"),
                meta().name("viewport").content("width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes"),
                title().child_text("moviefinder.app"),
                meta().name("description").content("Find movies and TV shows to watch"),
                link().rel("icon").type_("image/svg+xml").href("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 36 36'><text y='32' font-size='32'>🍿</text></svg>"),
                link().rel("stylesheet").href("./output.css"),
                // style().child_text_unsafe(include_str!("../../public/output.css")),
                link().rel("preconnect").href(TMDB_IMAGE_BASE_URL),
                script().src_datastar_cdn().defer(),
                script().src_drawer_element().defer(),
                script().src_swiper_cdn().defer(),
                script().src_image_element().defer(),
            ])
        )
        .child(
            body()
            .class("bg-black text-white flex flex-col items-center justify-center w-[100vw] h-[100dvh] max-h-[100dvh] overflow-hidden")            
            .child_signals_json(false)
            .data_on(|e| e.load().js(&Js::push_url(&self.url)))
            .child(
                div()
                    .class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col relative")
                    .child(Screen::default().view())
                    .child(Toast::view_root())
                    .child(Drawer::view_root())
            )
        )
    }
}
