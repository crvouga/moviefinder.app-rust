use crate::core::{dynamic_data::DynamicDataHashMap, session::session_id::SessionId};

#[derive(Debug, Clone)]
pub struct Req {
    pub session_id: SessionId,
    pub payload: DynamicDataHashMap,
}
