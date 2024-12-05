#![allow(dead_code)]
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
