use serde::{Deserialize, Serialize};

use crate::{
    core::{datastar::datastar::js_dot_value, posix::Posix, url_encoded},
    user::{user_id::UserId, username::Username},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: UserId,
    pub username: Username,
    pub created_at_posix: Posix,
    pub avatar_seed: Option<String>,
}

impl UserProfile {
    pub fn new(user_id: &UserId) -> Self {
        Self {
            user_id: user_id.clone(),
            username: Username::generate(),
            created_at_posix: Posix::now(),
            avatar_seed: Some(user_id.as_str().to_owned()),
        }
    }

    pub fn to_avatar_url(&self) -> String {
        let avatar_seed = self.avatar_seed.as_deref().unwrap_or_default();

        let avatar_url = to_avatar_url(avatar_seed);

        avatar_url
    }
}

const AVATAR_BASE_URL: &str = "https://api.dicebear.com/9.x/fun-emoji/svg?seed=";

pub fn js_avatar_url_signal(avatar_seed_signal: &str) -> String {
    format!(
        "`{}${{encodeURIComponent({})}}`",
        AVATAR_BASE_URL,
        js_dot_value(avatar_seed_signal)
    )
}

pub fn to_avatar_url(avatar_seed: &str) -> String {
    let avatar_url = AVATAR_BASE_URL.to_owned() + &url_encoded::encode(avatar_seed);

    avatar_url
}
