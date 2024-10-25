use crate::user_session::session_id::SessionId;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Req {
    pub session_id: SessionId,
    pub form_data: HashMap<String, String>,
}
