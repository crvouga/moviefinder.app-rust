use serde::{Deserialize, Serialize};

use crate::{
    core::{history::History, key_value_db::interface::KeyValueDbExt, unit_of_work::uow},
    ctx::Ctx,
    user::user_profile::user_profile_::UserProfile,
};

#[derive(Default, Serialize, Deserialize)]
pub struct FormState {
    pub history: History<String>,
}

fn to_key(profile: &UserProfile) -> String {
    format!("edit_profile:form_state:{}", profile.user_id.as_str())
}

impl FormState {
    pub async fn get(ctx: &Ctx, profile: &UserProfile) -> FormState {
        let key = to_key(&profile);

        ctx.key_value_db
            .get::<FormState>(&key)
            .await
            .unwrap_or_default()
            .unwrap_or_default()
    }

    pub async fn put(
        ctx: &Ctx,
        profile: &UserProfile,
        form_state: &FormState,
    ) -> Result<(), crate::core::error::CoreError> {
        let key = to_key(&profile);

        ctx.key_value_db.put(uow(), &key, form_state).await?;

        Ok(())
    }
}
