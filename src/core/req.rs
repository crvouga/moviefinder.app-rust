use super::params::ParamsHashMap;
use super::session::session_id::SessionId;

#[derive(Debug, Clone, PartialEq)]
pub struct Req {
    pub session_id: SessionId,
    pub params: ParamsHashMap,
}
