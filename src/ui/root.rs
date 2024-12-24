use crate::{
    core::{
        css::selector,
        datastar::datastar::fragments,
        dynamic_data::DynamicData,
        html::*,
        http::response_writer::ResponseWriter,
        js::Js,
        tmdb_api::TMDB_IMAGE_BASE_URL,
        ui::{drawer::Drawer, spinner_block::SpinnerBlock, toast::Toast},
    },
    req::Req,
};

fn css_selector_for_id(id: &str) -> String {
    format!(r#"[id="{}"]"#, id)
}

fn ensure_leading_slash(url: &str) -> String {
    if url.starts_with('/') {
        url.to_owned()
    } else {
        format!("/{}", url)
    }
}

fn dedupe<T: Eq + std::hash::Hash + Clone>(v: Vec<T>) -> Vec<T> {
    let mut seen = std::collections::HashSet::new();
    v.into_iter().filter(|e| seen.insert(e.clone())).collect()
}

impl ResponseWriter {
    pub async fn send_screen(
        &mut self,
        r: &Req,
        screen: Elem,
    ) -> Result<(), crate::core::error::Error> {
        let url = ensure_leading_slash(&r.url);

        let fallback: &Vec<String> = &Vec::new();
        let res_signal_loaded_screens = self
            .state
            .get_all("signal_loaded_screens")
            .unwrap_or(fallback);

        let fallback: &Vec<String> = &Vec::new();
        let signal_loaded_screens = r
            .payload
            .get_all("signal_loaded_screens")
            .unwrap_or(fallback);

        let signal_loaded_screens_prev = res_signal_loaded_screens
            .iter()
            .chain(signal_loaded_screens.iter())
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        let mut signal_loaded_screens_new = dedupe(
            signal_loaded_screens_prev
                .iter()
                .chain(std::iter::once(&url))
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        );

        signal_loaded_screens_new.sort();

        self.send_signal(
            "signal_loaded_screens",
            &serde_json::to_string(&signal_loaded_screens_new).unwrap_or("[]".to_owned()),
        )
        .await?;

        let screen = screen
            .id(&url)
            .data_show(&format!("signal_location.value === '{}'", url));

        if signal_loaded_screens_prev.contains(&url) {
            fragments(screen)
                .selector(&css_selector_for_id(&url))
                .send(self)
                .await?;
        } else {
            fragments(screen)
                .merge_mode_append()
                .selector(&selector::id("screen"))
                .send(self)
                .await?;
        }

        self.state.insert("signal_loaded_screens", url);

        Ok(())
    }
}

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
        .child(
            head().children(vec![
                meta().charset("UTF-8"),
                meta().name("viewport").content("width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes"),
                title().child_text("moviefinder.app"),
                meta().name("description").content("Find movies and TV shows to watch"),
                link().rel("icon").type_("image/svg+xml").href("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 36 36'><text y='32' font-size='32'>🍿</text></svg>"),
                link().rel("stylesheet").href("./output.css"),
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
            .data_signal_location()
            .child(
                div()
                    .class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col relative")
                    .child(
                        div()
                            .id("screen")
                            .class("w-full h-full flex flex-col overflow-hidden relative")
                            .data_on(|b| b.load().push_url(&self.url))
                            .child(SpinnerBlock::default().view().class("absolute inset-0 pointer-events-none").data_show("!signal_loaded_screens.value.includes(signal_location.value)"))
                    )
                    .child(Toast::view_root())
                    .child(Drawer::view_root())
            )
        )
    }
}

fn js_dispatch_location_changed(id: &str) -> String {
    r#"
    const update_signal_location = () => {
        const signal_location_root = document.getElementById('ROOT_ID');
        signal_location_root.dispatchEvent(new CustomEvent('signal_location', { detail: { value: location.pathname } }));
    };
    const original_push_state = history.pushState;
    history.pushState = (...args) => {
        original_push_state.apply(history, args);
        update_signal_location();
    };
    const original_replace_state = history.replaceState;
    history.replaceState = (...args) => {
        original_replace_state.apply(history, args);
        update_signal_location();
    };
    window.addEventListener('popstate', update_signal_location);
    window.addEventListener('pushstate', update_signal_location);
    window.addEventListener('replacestate', update_signal_location);
    "#.to_string().replace("ROOT_ID", id)
}

impl Elem {
    pub fn data_signal_location(self) -> Self {
        self.id("signal_location_root")
            .data_signal("signal_location", "location.pathname")
            .data_signal("signal_loaded_screens", "[]")
            .data_on(|e| {
                e.e("signal_location")
                    .js("signal_location.value = location.pathname")
                    .js(&Js::if_then_else(
                        "!signal_loaded_screens.value.includes(signal_location.value)",
                        &Js::sse("location.pathname"),
                        "null",
                    ))
            })
            .data_on(|e| {
                e.load()
                    .js(&js_dispatch_location_changed("signal_location_root"))
            })
    }
}
