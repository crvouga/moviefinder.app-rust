use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum InteractionAction {
    #[default]
    Add,
    Retract,
}

impl From<String> for InteractionAction {
    fn from(value: String) -> Self {
        let cleaned = value
            .trim()
            .to_lowercase()
            .replace(|c: char| c == '-' || c == '_', " ")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        match cleaned.as_str() {
            "add" => InteractionAction::Add,
            "retract" => InteractionAction::Retract,
            _ => InteractionAction::default(),
        }
    }
}
