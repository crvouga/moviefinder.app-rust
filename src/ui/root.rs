use crate::{
    core::{html::*, res::Res, tmdb_api::TMDB_IMAGE_BASE_URL},
    route::Route,
};

const ROOT_ID: &str = "root";

fn root_selector() -> String {
    format!("#{}", ROOT_ID)
}

impl Elem {
    pub fn root_swap(self) -> Self {
        self.hx_target(&root_selector()).hx_swap_inner_html()
    }

    pub fn root_swap_route(self, route: Route) -> Self {
        self.root_swap().hx_get(&route.encode())
    }

    pub fn root_push_route(self, route: Route) -> Self {
        self.root_swap_route(route).hx_push_url()
    }
}

impl Res {
    pub fn root_redirect(route: Route) -> Self {
        Res::redirect(route.encode().to_string(), root_selector())
    }
}

pub struct Root {
    children: Vec<Elem>,
}

impl Root {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn children(mut self, children: Vec<Elem>) -> Self {
        self.children = children;
        self
    }

    pub fn view(self) -> Elem {
        html()
        .lang("en")
        .child(
            head().children(vec![
                meta().charset("UTF-8"),
                meta().name("viewport").content("width=device-width, initial-scale=1"),
                title().child_text("moviefinder.app"),
                meta().name("description").content("Find movies and TV shows to watch"),
                link().rel("icon").type_("image/svg+xml").href("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 36 36'><text y='32' font-size='32'>🍿</text></svg>"),
                // 
                meta().name("htmx-config").content(r#"{"historyCacheSize": 0, "refreshOnHistoryMiss": true}"#),
                link().rel("preconnect").href(TMDB_IMAGE_BASE_URL),
                // style().child_unsafe_text(include_str!("../output.css")),
                link().rel("stylesheet").href("./output.css"),
                script().src_htmx().defer(),
                script().src_htmx_loading_states().defer(),
                script().js_htmx_preserve_state(),
                style().css_htmx_loading_states(),
                script().src_swiper().defer(),
                script().js_image_element(),
            ])
        )
        .child(
            body()
                .class("bg-black text-white flex flex-col items-center justify-center w-[100vw] h-[100dvh] max-h-[100dvh] overflow-hidden")
                .hx_ext_loading_states()
                .hx_ext_preserve_state()
                .child(
                    div()
                    .id(ROOT_ID)
                    .class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col")
                    .children(self.children)
                )
        )
    }
}
