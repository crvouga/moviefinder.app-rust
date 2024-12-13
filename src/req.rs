use crate::{
    core::{dynamic_data::DynamicDataHashMap, session::session_id::SessionId},
    ctx::Ctx,
    user::{user_id::UserId, user_profile::user_profile_::UserProfile},
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

    pub async fn profile(&self, ctx: &Ctx) -> Option<UserProfile> {
        let user_id = self.user_id(ctx).await?;

        let profile = ctx
            .user_profile_db
            .find_one_by_user_id(&user_id)
            .await
            .unwrap_or_default()?;

        Some(profile)
    }
}
