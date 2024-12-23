use serde::{Deserialize, Serialize};

use crate::core::uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MediaListId(String);

impl MediaListId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for MediaListId {
    fn default() -> Self {
        MediaListId::new(&format!("list-id-{}", uuid::v4()))
    }
}

impl From<String> for MediaListId {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}
