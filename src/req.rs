use crate::{
    core::{dynamic_data::DynamicDataHashMap, session::session_id::SessionId},
    ctx::Ctx,
    user::user_id::UserId,
};

#[derive(Debug, Clone)]
pub struct Req {
    pub session_id: SessionId,
    pub payload: DynamicDataHashMap,
}

impl Req {
    pub async fn user_id(&self, ctx: &Ctx) -> Option<UserId> {
        ctx.user_session_db
            .find_by_session_id(&self.session_id)
            .await
            .ok()
            .unwrap_or_default()
            .map(|s| s.user_id)
    }
}
