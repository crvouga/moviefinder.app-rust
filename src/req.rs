use crate::{core::http::form_data::FormData, user_session::session_id::SessionId};

#[derive(Debug, Clone, PartialEq)]
pub struct Req {
    pub session_id: SessionId,
    pub form_data: FormData,
}
