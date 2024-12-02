use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
