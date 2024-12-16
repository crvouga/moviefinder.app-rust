use serde::{Deserialize, Serialize};

use crate::core::random;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum InteractionAction {
    Add,
    Retract,
}

impl InteractionAction {
    pub fn random() -> Self {
        random::choice(vec![InteractionAction::Add, InteractionAction::Retract]).unwrap()
    }
}

impl InteractionAction {
    pub fn to_string(&self) -> String {
        match self {
            InteractionAction::Add => "Add".to_string(),
            InteractionAction::Retract => "Retract".to_string(),
        }
    }
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
