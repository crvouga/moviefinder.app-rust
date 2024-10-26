use crate::core::http::{
    request::HttpRequest,
    response::HttpResponse,
    response_cookie::{HttpResponseCookie, SameSite},
};
use futures::future::FutureExt;
use std::future::Future;
use std::pin::Pin;

use super::session_id::SessionId;

const SESSION_ID_COOKIE_NAME: &str = "session_id";

fn new_session_cookie(session_id: String) -> HttpResponseCookie {
    HttpResponseCookie {
        domain: None,
        expires: None,
        path: Some("/".to_string()),
        http_only: true,
        secure: true,
        max_age: Some(31536000),
        name: SESSION_ID_COOKIE_NAME.to_string(),
        value: session_id,
        same_site: Some(SameSite::Strict),
    }
}

pub fn wrap_session_id<F, Fut>(
    respond: F,
) -> impl Fn(HttpRequest) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Clone
where
    F: Fn(SessionId, HttpRequest) -> Fut + Send + Sync + 'static + Clone,
    Fut: Future<Output = HttpResponse> + Send + 'static,
{
    move |http_request: HttpRequest| {
        let cookie_session_id = http_request
            .cookies
            .get(SESSION_ID_COOKIE_NAME)
            .cloned()
            .unwrap_or("".to_string());

        let session_id = SessionId::new(&cookie_session_id).unwrap_or_default();

        let should_set_cookie = cookie_session_id != session_id.as_string();

        let fut = respond(session_id.clone(), http_request).map(move |mut http_response| {
            if should_set_cookie {
                let session_cookie = new_session_cookie((&session_id).as_str().to_string());

                http_response
                    .headers
                    .insert("Set-Cookie".to_string(), session_cookie.to_string());
            }

            http_response
        });

        Box::pin(fut)
    }
}
