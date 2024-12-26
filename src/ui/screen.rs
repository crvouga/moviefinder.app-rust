use crate::{
    core::{
        css::selector, datastar::datastar::fragments, dynamic_data::DynamicData, html::*,
        http::response_writer::ResponseWriter, js::Js, ui::spinner_block::SpinnerBlock,
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
        screen: Html,
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

#[derive(Default)]
pub struct Screen;

impl Screen {
    pub fn view(self) -> Html {
        main()
            .id("screen")
            .class("w-full h-full flex flex-col overflow-hidden relative")
            .data_signal("signal_location", "location.pathname")
            .data_signal("signal_loaded_screens", "[]")
            .data_signal("signal_preteched_screens", "[]")
            .data_on(|e| {
                e.e("event_location_changed")
                    .js("signal_location.value = location.pathname")
                    .js(&Js::if_then_else(
                        "!signal_loaded_screens.value.includes(signal_location.value)",
                        &Js::sse("location.pathname"),
                        "null",
                    ))
            })
            .data_on(|e| e.load().js(&js_dispatch_location_changed()))
            .child(
                SpinnerBlock::default()
                    .view()
                    .class("absolute inset-0 pointer-events-none")
                    .data_show("!signal_loaded_screens.value.includes(signal_location.value)"),
            )
    }
}

impl Html {
    pub fn preload_screen(self, _url: &str) -> Self {
        self
        // .data_on(|e| {
        //     e.load().js(&Js::if_then_else(
        //         &format!("!signal_preteched_screens.value.includes('{}')", url),
        //         &Js::statments(vec![
        //             format!("signal_preteched_screens.value = Array.from(new Set(signal_preteched_screens.value.concat(['{}')))", url),
        //             Js::sse(&Js::quote(url)),
        //         ]),
        //         "null",
        //     ))
        // })
    }
}

fn js_dispatch_location_changed() -> String {
    r#"
    const update_signal_location = () => {
        document.getElementById('screen').dispatchEvent(new CustomEvent('event_location_changed'));
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
    window.addEventListener('DOMContentLoaded', update_signal_location);
    "#
    .to_string()
}
