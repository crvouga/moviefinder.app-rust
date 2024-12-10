use crate::{
    core::{session::session_id::SessionId, unstructed_data::UnstructuredDataHashMap},
    user::user_id::UserId,
};

#[derive(Debug, Clone)]
pub struct Req {
    pub session_id: SessionId,
    pub user_id: Option<UserId>,
    pub params: UnstructuredDataHashMap,
}
