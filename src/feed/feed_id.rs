use serde::{Deserialize, Serialize};

use crate::core::uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FeedId(String);

impl FeedId {
    #[allow(dead_code)]
    pub fn new(id: String) -> Self {
        Self(id)
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for FeedId {
    fn default() -> Self {
        FeedId::new(uuid::v4().to_string())
    }
}
