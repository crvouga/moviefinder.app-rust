use serde::{Deserialize, Serialize};

use crate::core::uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub struct MediaId(String);

impl Default for MediaId {
    fn default() -> Self {
        MediaId::new(format!("media-{}", uuid::v4()))
    }
}

impl MediaId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for MediaId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
