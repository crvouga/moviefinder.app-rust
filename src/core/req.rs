use super::http::form_data::FormData;
use super::session::session_id::SessionId;

#[derive(Debug, Clone, PartialEq)]
pub struct Req {
    pub session_id: SessionId,
    pub form_data: FormData,
}
