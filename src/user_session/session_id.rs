use crate::core::uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(id: String) -> Option<Self> {
        let cleaned = id.trim().to_string();
        if cleaned.is_empty() {
            None
        } else {
            Some(Self(cleaned))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        SessionId(uuid::v4().to_string())
    }
}
