use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn generate() -> Self {
        Self("movielover-123".to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn from_string(username: &str) -> Self {
        Self(username.to_string())
    }
}
