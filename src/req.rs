use crate::{
    core::{dynamic_data::DynamicDataHashMap, session::session_id::SessionId},
    user::user_id::UserId,
};

#[derive(Debug, Clone)]
pub struct Req {
    pub session_id: SessionId,
    pub user_id: Option<UserId>,
    pub params: DynamicDataHashMap,
}
