use crate::core::uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MediaInteractionId(String);

impl MediaInteractionId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for MediaInteractionId {
    fn default() -> Self {
        MediaInteractionId::new(&format!("media-interaction-{}", uuid::v4()))
    }
}

impl From<String> for MediaInteractionId {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}
