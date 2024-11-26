use crate::core::http::{
    header::SetHeader,
    request::HttpRequest,
    response_cookie::{HttpResponseCookie, SameSite},
    response_writer::HttpResponseWriter,
};

use super::session_id::SessionId;

const SESSION_ID_COOKIE_NAME: &str = "session_id";

fn new_session_cookie(session_id: String) -> HttpResponseCookie {
    HttpResponseCookie {
        domain: None,
        expires: None,
        path: Some("/".to_string()),
        http_only: true,
        secure: false,
        max_age: Some(31536000),
        name: SESSION_ID_COOKIE_NAME.to_string(),
        value: session_id,
        same_site: Some(SameSite::Strict),
    }
}

pub fn write_session_id(
    request: &HttpRequest,
    response_writer: &mut HttpResponseWriter,
) -> SessionId {
    let cookie_session_id = request
        .cookies
        .get(SESSION_ID_COOKIE_NAME)
        .map_or("", |v| v.as_str());

    let session_id = SessionId::new(&cookie_session_id).unwrap_or_default();

    let should_set_cookie = cookie_session_id != session_id.as_string();

    if should_set_cookie {
        let session_cookie = new_session_cookie((&session_id).as_str().to_string());
        response_writer.set_header("Set-Cookie", &session_cookie.to_string());
    }

    session_id
}
