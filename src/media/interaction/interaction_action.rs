use crate::core::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum InteractionAction {
    Add,
    Retract,
}

impl InteractionAction {
    #[allow(dead_code)]
    pub fn random() -> Self {
        random::choice(vec![InteractionAction::Add, InteractionAction::Retract]).unwrap()
    }
}

impl InteractionAction {
    pub fn from_string(value: String) -> Option<Self> {
        let cleaned = value
            .trim()
            .to_lowercase()
            .replace(|c: char| c == '-' || c == '_', " ")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        match cleaned.as_str() {
            "add" => Some(InteractionAction::Add),
            "retract" => Some(InteractionAction::Retract),
            _ => None,
        }
    }
}
