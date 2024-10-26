use crate::core::{html::*, hx, ui};

const ROOT_ID: &str = "root";
pub const ROOT_SELECTOR: &str = "#root";

pub fn view_root(children: &[Elem]) -> Elem {
    html(&[lang("en")], &[
        head(&[
            meta(&[charset("UTF-8")]),
            meta(&[name("viewport"), content("width=device-width, initial-scale=1")]),
            title("moviefinder.app"),
            meta(&[name("description"), content("Find movies and TV shows to watch")]),
            link(&[
                rel("icon"),
                attr("type", "image/svg+xml"),
                href("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 36 36'><text y='32' font-size='32'>🍿</text></svg>")
            ]),
            link(&[rel("preconnect"),href("https://image.tmdb.org")]),
            script(&[src("https://cdn.tailwindcss.com")],  &[]),
            script(&[src("https://unpkg.com/htmx.org@2.0.1"),defer("true")],  &[]),
            script(&[src("https://unpkg.com/htmx-ext-preload@2.0.1/preload.js"),defer("true")],  &[]),
            script(&[src("https://cdn.jsdelivr.net/npm/swiper@11/swiper-element-bundle.min.js"),defer("true")], &[]),
            ui::image::script_image_element(),
        ]),
        body(
            &[
                class("bg-black text-white flex flex-col items-center justify-center w-[100vw] h-[100dvh] max-h-[100dvh] overflow-hidden"),
                hx::ext("preload")
            ],
            &[
                div(
                    &[
                        id(ROOT_ID),
                        class("h-full max-h-[915px] w-full max-w-[520px] border box-border rounded overflow-hidden flex flex-col"),
                        hx::boost(),
                    ],
                    children
                ),
            ]
        ),
    ])
}
