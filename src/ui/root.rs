use crate::{
    core::{html::*, res::Res, ui::image::Image},
    route::Route,
};

const ROOT_ID: &str = "root";
const ROOT_SELECTOR: &str = "#root";

impl Elem {
    pub fn root_swap_screen(self, route: Route) -> Self {
        self.hx_target(ROOT_SELECTOR)
            .hx_swap_inner_html()
            .hx_get(&route.encode())
    }

    pub fn root_push_screen(self, route: Route) -> Self {
        self.root_swap_screen(route).hx_push_url()
    }

    pub fn root_replace_screen(self, route: Route) -> Self {
        self.root_swap_screen(route).hx_replace_url()
    }
}

impl Res {
    pub fn root_redirect_screen(route: Route) -> Self {
        Res::redirect(route.encode().to_string(), ROOT_SELECTOR.to_string())
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
                // meta().name("htmx-config").content("{&quot;historyCacheSize&quot;: 0, &quot;refreshOnHistoryMiss&quot;: true}"),
                link().rel("preconnect").href("https://image.tmdb.org"),
                script().src("https://cdn.tailwindcss.com"),
                script().child_unsafe_text(r#"
                tailwind.config = {
                    theme: {
                        extend: {
                            borderColor: {
                                DEFAULT: '#3f3f46',
                            },
                        },
                    },
                }
                "#),
                script().src("https://unpkg.com/htmx.org@2.0.1").defer(),
                script().src("https://unpkg.com/htmx-ext-preload@2.0.1/preload.js").defer(),
                script().src("https://unpkg.com/htmx.org@1.9.12/dist/ext/loading-states.js").defer(),
                script().src("https://cdn.jsdelivr.net/npm/swiper@11/swiper-element-bundle.min.js").defer(),
                Image::script(),
            ])
        )
        .child(
            body()
                .class("bg-black text-white flex flex-col items-center justify-center w-[100vw] h-[100dvh] max-h-[100dvh] overflow-hidden")
                .hx_ext(vec!["preload", "loading-states"])
                .hx_boost()
                .child(
                    div()
                    .id(ROOT_ID)
                    .class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col")
                    .children(self.children)
                )
        )
    }
}
