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

    pub fn to_string(&self) -> String {
        self.0.clone()
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
