use crate::html::*;
use crate::hx;
use crate::route;
use crate::ui;

const ROOT_ID: &'static str = "app";
pub const ROOT_SELECTOR: &'static str = "#app";

pub fn view_root(route: &route::Route) -> Elem {
    return html(&[
        head(&[
            meta(&[charset("UTF-8")]),
            meta(&[name("viewport"), content("width=device-width, initial-scale=1")]),
            link(&[
                rel("stylesheet"),
                href("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 36 36'><text y='32' font-size='32'>🍿</text></svg>")
            ]),
            script(&[src("https://cdn.tailwindcss.com")], ""),
            script(&[src("https://unpkg.com/htmx.org@2.0.1")], ""),
            script(&[src("https://unpkg.com/htmx-ext-preload@2.0.1/preload.js")], ""),

        ]),
        body(
            &[
                class("bg-black text-white flex flex-col items-center justify-center w-[100vw] h-[100dvh] max-h-[100dvh]"),
                hx::ext("preload")
            ],
            &[
                div(
                    &[
                        id(ROOT_ID),
                        class("w-full max-w-[500px] h-full max-h-[800px] border rounded overflow-hidden flex flex-col"),
                        hx::get(&route.encode()),
                        hx::Trigger::Load.attr(),
                        hx::boost(),
                    ],
                    &[
                        div(&[class("w-full h-full flex items-center justify-center")], &[
                            ui::icon::spinner(
                                &[class("size-16 animate-spin")]
                            ),
                        ]),
                    ]
                ),
            ]
        ),
    ]);
}
