use crate::{
    core::{html::*, tmdb_api::TMDB_IMAGE_BASE_URL},
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
}

pub struct Root {
    route: Route,
}

impl Root {
    pub fn new(route: Route) -> Self {
        Self { route }
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
                link().rel("stylesheet").href("./output.css").type_("text/css").on_load("this.media='all'").media_print(),
                link().rel("preconnect").href(TMDB_IMAGE_BASE_URL),
                // script().src("https://cdn.tailwindcss.com/"),
                // script().child_text_unsafe(r#"""
                // tailwind.config = {
                //     theme: {
                //         extend: {
                //             borderColor: {
                //                 DEFAULT: '#3f3f46',
                //             },
                //         },
                //     },
                //     plugins: [],
                // }
                // "#),
                script().src_head_injector().defer(),
                script().src_datastar().defer(),
                script().child_text_unsafe("window.addEventListener('popstate', () => location.reload());")
            ])
        )
        .child(
            body()
            .class("bg-black text-white flex flex-col items-center justify-center w-[100vw] h-[100dvh] max-h-[100dvh] overflow-hidden")
            .style("background-color: #000;")
            .child(
                div()
                    .class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col")
                    .child(
                        div().id_root().data_on_load_get(&self.route.encode())
                    )
            )
        )
    }
}
