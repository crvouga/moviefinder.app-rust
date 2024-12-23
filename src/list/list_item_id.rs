use serde::{Deserialize, Serialize};

use crate::core::uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MediaListItemId(String);

impl MediaListItemId {
    pub fn from_string(id: &str) -> Self {
        Self(id.to_string())
    }

    // pub fn as_str(&self) -> &str {
    //     &self.0
    // }
}

impl Default for MediaListItemId {
    fn default() -> Self {
        MediaListItemId::from_string(&format!("list-item-id-{}", uuid::v4()))
    }
}

impl From<String> for MediaListItemId {
    fn from(value: String) -> Self {
        Self::from_string(&value)
    }
}
