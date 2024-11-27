use super::session_id::SessionId;
use crate::core::http::{
    request::Request,
    response_cookie::{ResponseCookie, SameSite},
    response_writer::ResponseWriter,
    set_header::SetHeader,
};

impl Request {
    pub fn maybe_session_id(&self) -> Option<&str> {
        self.cookies.get(SESSION_ID_COOKIE_NAME).map(|v| v.as_str())
    }

    pub fn session_id(&self) -> SessionId {
        let cookie_session_id = self.maybe_session_id().unwrap_or("");

        SessionId::new(cookie_session_id).unwrap_or_default()
    }

    pub fn write_session_id_cookie(&self, w: &mut ResponseWriter) {
        let session_id = self.session_id();

        let should_set_cookie = self.maybe_session_id().unwrap_or("") != session_id.as_string();

        if should_set_cookie {
            let session_cookie = new_session_cookie(session_id.as_string());
            w.set_header("Set-Cookie", &session_cookie.to_string());
        }
    }
}

const SESSION_ID_COOKIE_NAME: &str = "session_id";

fn new_session_cookie(session_id: String) -> ResponseCookie {
    ResponseCookie {
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
