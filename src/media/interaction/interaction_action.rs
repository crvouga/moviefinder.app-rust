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
    pub fn to_postgres_enum(&self) -> String {
        match self {
            InteractionAction::Add => "add".to_string(),
            InteractionAction::Retract => "retract".to_string(),
        }
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

        if cleaned.contains("retract") {
            return Some(InteractionAction::Retract);
        }

        if cleaned.contains("add") {
            return Some(InteractionAction::Add);
        }

        None
    }
}

pub fn to_all_interaction_actions() -> Vec<InteractionAction> {
    vec![InteractionAction::Add, InteractionAction::Retract]
}
