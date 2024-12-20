use serde::{Deserialize, Serialize};

use crate::core::uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ListId(String);

impl ListId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    // pub fn as_str(&self) -> &str {
    //     &self.0
    // }
}

impl Default for ListId {
    fn default() -> Self {
        ListId::new(&format!("list-id-{}", uuid::v4()))
    }
}

impl From<String> for ListId {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}
