#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use crate::core::{html::Elem, random, ui::icon};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub enum InteractionName {
    Liked,
    Disliked,
    Interested,
    NotInterested,
    Seen,
    NotSeen,
}

impl InteractionName {
    pub fn view_icon(&self, selected: bool, class: &str) -> Elem {
        match self {
            InteractionName::Liked => match selected {
                true => icon::solid::thumbs_up(class),
                false => icon::outlined::thumbs_up(class),
            },
            InteractionName::Disliked => match selected {
                true => icon::solid::thumbs_down(class),
                false => icon::outlined::thumbs_down(class),
            },
            InteractionName::Interested => icon::solid::check(class),
            InteractionName::NotInterested => icon::solid::x_mark(class),
            InteractionName::Seen => match selected {
                true => icon::solid::eye(class),
                false => icon::outlined::eye(class),
            },
            InteractionName::NotSeen => match selected {
                true => icon::solid::eye_slash(class),
                false => icon::outlined::eye_slash(class),
            },
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            InteractionName::Liked => "Liked".to_string(),
            InteractionName::Disliked => "Disliked".to_string(),
            InteractionName::Interested => "Looks Good".to_string(),
            InteractionName::NotInterested => "Looks Bad".to_string(),
            InteractionName::Seen => "Seen".to_string(),
            InteractionName::NotSeen => "Not Seen".to_string(),
        }
    }

    pub fn random() -> Self {
        let choice = random::choice(vec![
            InteractionName::Liked,
            InteractionName::Disliked,
            InteractionName::Interested,
            InteractionName::NotInterested,
            InteractionName::Seen,
            InteractionName::NotSeen,
        ])
        .unwrap();

        choice
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
            "liked" => Some(InteractionName::Liked),
            "disliked" => Some(InteractionName::Disliked),
            "interested" => Some(InteractionName::Interested),
            "not interested" => Some(InteractionName::NotInterested),
            "seen" => Some(InteractionName::Seen),
            "not seen" => Some(InteractionName::NotSeen),
            _ => None,
        }
    }
}

pub fn to_max_display_string_length() -> usize {
    vec![
        InteractionName::Liked,
        InteractionName::Disliked,
        InteractionName::Interested,
        InteractionName::NotInterested,
        InteractionName::Seen,
        InteractionName::NotSeen,
    ]
    .iter()
    .map(|name| name.to_display_string().len())
    .max()
    .unwrap_or_default()
}
