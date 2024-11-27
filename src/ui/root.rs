use crate::{
    core::{html::*, tmdb_api::TMDB_IMAGE_BASE_URL, ui},
    route::Route,
};

const ROOT_ID: &str = "root";

fn root_selector() -> String {
    format!("#{}", ROOT_ID)
}

impl Elem {
    pub fn id_root(self) -> Self {
        self.id(ROOT_ID)
    }

    pub fn hx_swap_root(self) -> Self {
        self.hx_target(&root_selector()).hx_swap_inner_html()
    }

    pub fn hx_swap_root_route(self, route: Route) -> Self {
        self.hx_swap_root().hx_get(&route.encode())
    }

    pub fn hx_push_root_route(self, route: Route) -> Self {
        self.hx_swap_root_route(route).hx_push_url()
    }
}

pub struct Root {
    children: Vec<Elem>,
    route: Route,
}

impl Root {
    pub fn new(route: Route) -> Self {
        Self {
            children: vec![],
            route,
        }
    }

    pub fn view(self) -> Elem {
        html()
        .lang("en")
        .child(
            head().children(vec![
                meta().charset("UTF-8"),
                meta().name("viewport").content("width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes"),
                title().child_text("moviefinder.app"),
                meta().name("description").content("Find movies and TV shows to watch"),
                link().rel("icon").type_("image/svg+xml").href("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 36 36'><text y='32' font-size='32'>🍿</text></svg>"),
                // link().rel("preconnect").href("https://fonts.googleapis.com"),
                // link().rel("preconnect").href("https://fonts.gstatic.com").crossorigin(),
                // link().href("https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap").rel("stylesheet"),
                link().rel("stylesheet").href("./output.css"),
                link().rel("preconnect").href(TMDB_IMAGE_BASE_URL),
                // meta().name("htmx-config").content(r#"{"historyCacheSize": 0, "refreshOnHistoryMiss": true}"#),
                // style().child_unsafe_text(include_str!("../output.css")),
                // script().src_htmx().defer(),
                // script().src_htmx_loading_states().defer(),
                // script().js_htmx_preserve_state(),
                // style().css_htmx_loading_states(),
                script().src_swiper().defer(),
                script().js_image_element(),
                // 
                script().src_datastar().defer(),
                script().child_unsafe_text(r#"
                    window.addEventListener('popstate', location.reload);
                "#)
            ])
        )
        .child(
            body()
                .class("bg-black text-white flex flex-col items-center justify-center w-[100vw] h-[100dvh] max-h-[100dvh] overflow-hidden")
                .child(
                    div().class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col").child(
                        ui::spinner_page::view()
                        .id_root()
                        .children(self.children)
                        .data_on_load_get(&self.route.encode())
                    )
                )
        )
    }
}
