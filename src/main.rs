use html::*;

mod feed;
mod html;
mod http;
mod res;
mod respond;
mod route;

pub fn view_root(children: Elem) -> Elem {
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
        ]),
        body(
            &[class("bg-black text-white flex flex-col items-center justify-center w-full h-[100dvh] max-h-[100dvh]")],
            &[
                div(
                    &[
                        id("app"), class("w-full max-w-[500px] h-full max-h-[800px] border rounded overflow-hidden")
                    ],
                    &[
                        div(&[class("w-full h-full flex items-center justify-center")], &[
                            children
                        ]),
                    ]
                ),
            ]
        ),
    ]);
}

fn respond(req: http::Request) -> http::Response {
    let route = route::decode(req.path);

    println!("{} {:?}", req.method, route);

    let is_hx_request = req.headers.iter().any(|(key, _value)| key == "HX-Request");

    if is_hx_request {
        let response = respond::respond(route);

        let html_response = res::to_http_response(response);

        return html_response;
    }

    let html = view_root(div(&[], &[text("Hello, world!")])).render();

    let response = res::Res::Html(html);

    let http_respond = res::to_http_response(response);

    println!("{:?}", http_respond);

    return http_respond;
}

fn main() {
    // read PORT environment variable
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = "127.0.0.1:".to_owned() + &port.to_string();
    println!("Listening on http://localhost:{}", port);
    http::start_server(&address, respond);
}
