use std::{collections::HashMap, io::Cursor};

use crate::{
    core::{
        html::{div, img, script, Elem},
        http::request::HttpRequest,
        res::Res,
        url::Url,
    },
    req::Req,
    route,
};

use image::{self, imageops::FilterType};

use super::{ctx::Ctx, route::Route};

pub async fn response(ctx: &Ctx, route: &Route, req: Req) -> Res {
    match route {
        Route::Resize => {
            let fallback = Res::image(vec![]);
            let width = req
                .form_data
                .get_first("width")
                .unwrap_or(&"".to_string())
                .parse::<u32>()
                .unwrap_or_default();

            if width == 0 {
                println!("Width is 0");
                return fallback;
            }

            let height = req
                .form_data
                .get_first("height")
                .unwrap_or(&"".to_string())
                .parse::<u32>()
                .unwrap_or_default();

            if height == 0 {
                println!("Height is 0");
                return fallback;
            }

            let src_empty = String::new();

            let src = req
                .form_data
                .get_first("src")
                .to_owned()
                .unwrap_or(&src_empty)
                .trim();

            if src.is_empty() {
                println!("Src is empty");
                return fallback;
            }

            let mut headers = HashMap::new();

            headers.insert(
                "Referer".to_string(),
                "https://www.themoviedb.org/".to_string(),
            );

            let request = HttpRequest {
                url: Url::from_str(src).unwrap_or_default(),
                body: vec![],
                cookies: Default::default(),
                form_data: Default::default(),
                headers,
                method: "GET".to_string(),
            };

            println!("Request: {:?}", request);

            let sent = ctx.http_client.send(request).await;

            let response = match sent {
                Ok(response) => response,
                Err(err) => {
                    println!("Error sending request {}", err);
                    return fallback;
                }
            };

            println!("Response: {:?}", response.clone().to_body_string());

            let image = image::load_from_memory(&response.body);

            let image = match image {
                Ok(image) => image,
                Err(err) => {
                    println!("Error loading image {}", err);
                    return fallback;
                }
            };

            let resized = image.resize(width, height, FilterType::Lanczos3);

            let mut buffer = Vec::new();

            resized
                .write_to(
                    &mut Cursor::new(&mut buffer),
                    image::ImageOutputFormat::Jpeg(80),
                )
                .unwrap_or(());

            Res::image(buffer)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LoadingImage {
    src: String,
}

impl LoadingImage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn src(mut self, src: &str) -> Self {
        self.src = src.to_string();
        self
    }

    fn is_valid_src(&self) -> bool {
        self.src.trim().len() > 0
    }

    pub fn view(self) -> Elem {
        let class = "w-full h-full bg-neutral-700 animate-pulse border-none";
        if self.is_valid_src() {
            img()
                .class(class)
                .on_load("this.classList.remove('animate-pulse')")
                .src(&self.src)
        } else {
            div().class(class)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ResizableImage {
    src: String,
}

impl ResizableImage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn src(mut self, src: &str) -> Self {
        self.src = src.to_string();
        self
    }

    pub fn view(self) -> Elem {
        LoadingImage::new()
            .view()
            .id("resizable-image")
            .hx_post(&route::Route::ResizableImage(Route::Resize).encode())
            .hx_trigger_intersect()
            .attr("data-src", &self.src) // Store the image source
            .hx_vals("js:{ ...getVals(event) }")
            .hx_target_this()
            .hx_swap_outer_html()
            .child(script().child_unsafe_text(
                r#"
                function getVals(event) {
                    const el = document.getElementById('resizable-image');
                    return {
                        width: Math.round(el.offsetWidth),
                        height: Math.round(el.offsetHeight),
                        src: el?.getAttribute?.('data-src') ?? ''
                    };
                }
                "#,
            ))
    }
}
