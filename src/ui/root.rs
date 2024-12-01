use crate::{
    core::{html::*, http::response_writer::ResponseWriter, tmdb_api::TMDB_IMAGE_BASE_URL},
    route::Route,
};

const ID_SCREEN: &str = "screen";

impl ResponseWriter {
    pub async fn send_screen_frag(&mut self, screen: Elem) -> Result<(), std::io::Error> {
        self.send_frag(screen.id(ID_SCREEN)).await?;

        Ok(())
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
                link().rel("stylesheet").href("./output.css"),
                link().rel("preconnect").href(TMDB_IMAGE_BASE_URL),
                script().src_head_injector().defer(),
                script().src_datastar().defer(),
                script().child_text_unsafe("window.addEventListener('popstate', () => location.reload());")
            ])
        )
        .child(
            body()
            .class("bg-black text-white flex flex-col items-center justify-center w-[100vw] h-[100dvh] max-h-[100dvh] overflow-hidden")            
            .child(
                div()
                    .class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col")
                    .child(
                        div().id(ID_SCREEN).data_on(|b| b.load().push_then_get(&self.route.encode()))
                    )
            )
        )
    }
}
