use crate::core::uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(id: &str) -> Option<Self> {
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

    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

impl Default for SessionId {
    fn default() -> Self {
        SessionId(uuid::v4().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = SessionId::new(&"  ");
        assert!(id.is_none());

        let id = SessionId::new(&"  123  ").unwrap();
        assert_eq!(id.as_str(), "123");
    }

    #[test]
    fn test_initial() {
        let id = "";
        let session_id = SessionId::new(&id);
        assert_eq!(session_id, None);
    }
}
