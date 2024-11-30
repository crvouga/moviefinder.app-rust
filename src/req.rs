use crate::core::{params::ParamsHashMap, session::session_id::SessionId};

#[derive(Debug, Clone)]
pub struct Req {
    pub session_id: SessionId,
    pub params: ParamsHashMap,
    pub path: String,
}
