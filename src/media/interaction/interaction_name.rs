use serde::{Deserialize, Serialize};

use crate::core::html::{div, Elem};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum InteractionName {
    Liked,
    Disliked,
    Interested,
    NotInterested,
    Seen,
    NotSeen,
}

impl InteractionName {
    pub fn view_icon(&self) -> Elem {
        match self {
            InteractionName::Liked => div(),
            InteractionName::Disliked => div(),
            InteractionName::Interested => div(),
            InteractionName::NotInterested => div(),
            InteractionName::Seen => div(),
            InteractionName::NotSeen => div(),
        }
    }

    pub fn to_name(&self) -> String {
        match self {
            InteractionName::Liked => "Liked".to_string(),
            InteractionName::Disliked => "Disliked".to_string(),
            InteractionName::Interested => "Interested".to_string(),
            InteractionName::NotInterested => "Not Interested".to_string(),
            InteractionName::Seen => "Seen".to_string(),
            InteractionName::NotSeen => "Not Seen".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InteractionName2 {
    id: String,
    icon: String,
    name: String,
    order: i32,
}

fn interaction_names() -> Vec<InteractionName2> {
    vec![
        InteractionName2 {
            id: "liked".to_string(),
            icon: "like".to_string(),
            name: "Liked".to_string(),
            order: 1,
        },
        InteractionName2 {
            id: "disliked".to_string(),
            icon: "dislike".to_string(),
            name: "Disliked".to_string(),
            order: 2,
        },
        InteractionName2 {
            id: "interested".to_string(),
            icon: "star".to_string(),
            name: "Interested".to_string(),
            order: 3,
        },
        InteractionName2 {
            id: "not-interested".to_string(),
            icon: "star".to_string(),
            name: "Not Interested".to_string(),
            order: 4,
        },
        InteractionName2 {
            id: "seen".to_string(),
            icon: "eye".to_string(),
            name: "Seen".to_string(),
            order: 5,
        },
        InteractionName2 {
            id: "not-seen".to_string(),
            icon: "eye-slash".to_string(),
            name: "Not Seen".to_string(),
            order: 6,
        },
    ]
}
