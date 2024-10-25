use crate::core::uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(id: String) -> Option<Self> {
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
}

impl Default for SessionId {
    fn default() -> Self {
        SessionId(uuid::v4().to_string())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = SessionId::new("  ".to_string());
        assert!(id.is_none());

        let id = SessionId::new("  123  ".to_string()).unwrap();
        assert_eq!(id.as_str(), "123");
    }

    #[test]
    fn test_initial() {
        let id = "".to_string();
        let session_id = SessionId::new(id);
        assert_eq!(session_id, None);
    }
}
