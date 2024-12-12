use crate::{
    core::{posix::Posix, url_encoded},
    user::{user_id::UserId, username::Username},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserProfile {
    pub user_id: UserId,
    pub username: Username,
    pub created_at_posix: Posix,
    pub avatar_seed: Option<String>,
    pub full_name: Option<String>,
}

impl UserProfile {
    pub fn new(user_id: &UserId) -> Self {
        let username = Username::generate();
        Self {
            user_id: user_id.clone(),
            avatar_seed: Some(username.to_string()),
            created_at_posix: Posix::now(),
            full_name: None,
            username,
        }
    }

    pub fn to_avatar_url(&self) -> String {
        let avatar_seed = self.avatar_seed.as_deref().unwrap_or_default();

        let avatar_url = to_avatar_url(avatar_seed);

        avatar_url
    }
}

const AVATAR_BASE_URL: &str = "https://api.dicebear.com/9.x/fun-emoji/svg?seed=";

pub fn js_avatar_url(avatar_seed_signal: &str) -> String {
    format!(
        "`{}${{encodeURIComponent({})}}`",
        AVATAR_BASE_URL, avatar_seed_signal
    )
}

pub fn to_avatar_url(avatar_seed: &str) -> String {
    let avatar_url = AVATAR_BASE_URL.to_owned() + &url_encoded::encode(avatar_seed);

    avatar_url
}
