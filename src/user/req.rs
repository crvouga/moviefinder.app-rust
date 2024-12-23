use crate::{
    ctx::Ctx,
    req::Req,
    user::{user_id::UserId, user_profile::user_profile_::UserProfile},
};

impl Req {
    pub async fn user_id(&self, ctx: &Ctx) -> Result<UserId, crate::core::error::Error> {
        ctx.user_session_db
            .find_by_session_id(&self.session_id)
            .await
            .ok()
            .unwrap_or_default()
            .map(|s| s.user_id)
            .ok_or(crate::core::error::Error::new("Not found"))
    }

    pub async fn user_profile(&self, ctx: &Ctx) -> Result<UserProfile, crate::core::error::Error> {
        let user_id = self.user_id(ctx).await?;

        ctx.user_profile_db
            .find_one_by_user_id(&user_id)
            .await
            .unwrap_or_default()
            .ok_or(crate::core::error::Error::new("Not found"))
    }
}
