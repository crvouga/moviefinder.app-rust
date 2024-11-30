use std::fmt::format;

use crate::{
    core::{
        html::*,
        http::{
            response_writer::ResponseWriter,
            server_sent_event::{sse, ServerSentEvent},
        },
        params::Params,
        tmdb_api::TMDB_IMAGE_BASE_URL,
    },
    req::Req,
    route::Route,
};

const ID_ROOT: &str = "root";

pub struct Root {
    route: Route,
}

impl ServerSentEvent {
    pub async fn send_screen(
        &self,
        _r: &Req,
        w: &mut ResponseWriter,
        _screen_id: &str,
        screen: Elem,
    ) -> Result<(), std::io::Error> {
        sse()
            .event_merge_fragments()
            .data_fragments(screen.id(ID_ROOT))
            .send(w)
            .await?;

        Ok(())
    }

    pub async fn _send_screen(
        &self,
        r: &Req,
        w: &mut ResponseWriter,
        screen_id: &str,
        screen: Elem,
    ) -> Result<(), std::io::Error> {
        let loaded = r
            .params
            .get_all("signalLoadedPaths")
            .map_or(vec![], |s| s.to_owned());

        sse()
            .event_merge_signals()
            .data_signals(&format!("{{signalScreenId: '{}'}}", screen_id))
            .send(w)
            .await?;

        if !loaded.contains(&screen_id.to_string()) {
            let js_add_loaded_path = format!(
                "$signalLoadedPaths = [...$signalLoadedPaths.filter(x => x !== '{}'), '{}'];",
                screen_id, screen_id
            );

            sse()
                .event_merge_fragments()
                .data_selector_id(ID_ROOT)
                .data_merge_mode_before()
                .data_fragments(
                    screen
                        .id(&screen_id)
                        .data_show(&format!("$signalScreenId === '{}' ", &screen_id))
                        .data_on_load(&js_add_loaded_path)
                        .id(screen_id),
                )
                .send(w)
                .await?;
        }

        Ok(())
    }
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
            .data_store("{signalScreenId: '', signalPath: '', signalLoadedPaths: []}")
            .data_on_store_change("window.ctx = ctx;")
            .child(
                script().child_text_unsafe(
                    r#"
                    (function () {
                        const originalPushState = history.pushState;
                        const originalReplaceState = history.replaceState;

                        function triggerCustomEvent(type) {
                            const event = new Event(type);
                            window.dispatchEvent(event);
                        }

                        history.pushState = function (...args) {
                            originalPushState.apply(this, args);
                            triggerCustomEvent('pushstate');
                        };

                        history.replaceState = function (...args) {
                            originalReplaceState.apply(this, args);
                            triggerCustomEvent('replacestate');
                        };

                        function onChange() {
                            if (typeof window.ctx === 'undefined') {
                                return;
                            }
                            ctx.store().signalPath.value = location.pathname;
                        }

                        window.addEventListener('hashchange', onChange);
                        window.addEventListener('popstate', onChange);
                        window.addEventListener('pushstate', onChange);
                        window.addEventListener('replacestate', onChange);
                        window.addEventListener('load', onChange);
                        window.addEventListener('DOMContentLoaded', onChange);
                    })();
                    "#
                )
            )
            // .child_debug_store()
            .child(
                div()
                    .class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col")
                    .child(
                        div().id(ID_ROOT).data_on_load_get(&self.route.encode())
                    )
            )
        )
    }
}
