use super::{
    request::HttpRequest,
    response::HttpResponse,
    response_cookie::{HttpResponseCookie, SameSite},
};
use futures::future::FutureExt;
use std::future::Future;
use std::pin::Pin;

const SESSION_COOKIE_NAME: &str = "session_id";

fn new_session_cookie(session_id: &str) -> HttpResponseCookie {
    HttpResponseCookie {
        domain: None,
        expires: Some("0".to_string()),
        path: Some("/".to_string()),
        http_only: true,
        secure: true,
        max_age: Some(31536000),
        name: SESSION_COOKIE_NAME.to_string(),
        value: session_id.to_string(),
        same_site: Some(SameSite::Strict),
    }
}

pub fn wrap_session_id<F, Fut>(
    respond: F,
) -> impl Fn(HttpRequest) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Clone
where
    F: Fn(&str, HttpRequest) -> Fut + Send + Sync + 'static + Clone,
    Fut: Future<Output = HttpResponse> + Send + 'static,
{
    move |http_request: HttpRequest| {
        let maybe_session_id = http_request.cookies.get(SESSION_COOKIE_NAME).cloned();
        let session_id = maybe_session_id.clone().unwrap_or_default();

        let fut = respond(&session_id, http_request).map(move |mut http_response| {
            if maybe_session_id.is_none() {
                let session_cookie = new_session_cookie(&session_id);

                http_response
                    .headers
                    .insert("Set-Cookie".to_string(), session_cookie.to_string());
            }

            http_response
        });

        Box::pin(fut)
    }
}
