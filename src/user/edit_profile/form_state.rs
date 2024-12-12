use serde::{Deserialize, Serialize};

use crate::{
    core::{history::History, unit_of_work::uow},
    ctx::Ctx,
    user::user_profile::user_profile_::UserProfile,
};

#[derive(Default, Serialize, Deserialize)]
pub struct FormState {
    pub avatar_seed: History<String>,
}

fn to_key(profile: &UserProfile) -> String {
    format!("edit_profile:form_state:{}", profile.user_id.as_str())
}

impl FormState {
    pub async fn get(ctx: &Ctx, profile: &UserProfile) -> FormState {
        let key = to_key(&profile);

        let got = ctx
            .key_value_db
            .get(&key)
            .await
            .unwrap_or_default()
            .unwrap_or_default();

        let decoded: FormState = serde_json::from_str(&got).unwrap_or_default();

        decoded
    }

    pub async fn put(
        ctx: &Ctx,
        profile: &UserProfile,
        form_state: &FormState,
    ) -> Result<(), std::io::Error> {
        let key = to_key(&profile);

        let encoded = serde_json::to_string(form_state).unwrap();

        ctx.key_value_db.put(uow(), &key, encoded).await?;

        Ok(())
    }

    pub fn reset(&mut self, profile: &UserProfile) {
        self.avatar_seed = History::default();
        self.avatar_seed
            .push(profile.avatar_seed.clone().unwrap_or_default());
    }
}
