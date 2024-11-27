use crate::core::params::ParamsHashMap;
use crate::core::session::session_id::SessionId;

#[derive(Debug, Clone, PartialEq)]
pub struct _Req {
    pub session_id: SessionId,
    pub params: ParamsHashMap,
}
