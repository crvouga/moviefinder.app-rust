use crate::core::{html::Html, random, ui::icon};
use serde::{Deserialize, Serialize};

pub mod postgres;

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
    pub fn view_icon(&self, selected: bool, class: &str) -> Html {
        match (self, selected) {
            (InteractionName::Liked, true) => icon::solid::thumbs_up(class),
            (InteractionName::Liked, false) => icon::outlined::thumbs_up(class),
            (InteractionName::Disliked, true) => icon::solid::thumbs_down(class),
            (InteractionName::Disliked, false) => icon::outlined::thumbs_down(class),
            (InteractionName::Interested, _) => icon::solid::check(class),
            (InteractionName::NotInterested, _) => icon::solid::x_mark(class),
            (InteractionName::Seen, true) => icon::solid::eye(class),
            (InteractionName::Seen, false) => icon::outlined::eye(class),
            (InteractionName::NotSeen, true) => icon::solid::eye_slash(class),
            (InteractionName::NotSeen, false) => icon::outlined::eye_slash(class),
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

    pub fn to_machine_string(&self) -> String {
        match self {
            InteractionName::Liked => "liked".to_string(),
            InteractionName::Disliked => "disliked".to_string(),
            InteractionName::Seen => "seen".to_string(),
            InteractionName::NotSeen => "not-seen".to_string(),
            InteractionName::Interested => "interested".to_string(),
            InteractionName::NotInterested => "not-interested".to_string(),
        }
    }

    pub fn random() -> Self {
        random::choice(to_all_interaction_names()).unwrap_or(InteractionName::Liked)
    }

    pub fn from_string(value: String) -> Option<Self> {
        let cleaned = value
            .trim()
            .to_lowercase()
            .replace(|c: char| c == '-' || c == '_', " ")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        if cleaned.contains("like") {
            return Some(InteractionName::Liked);
        }

        if cleaned.contains("dislike") {
            return Some(InteractionName::Disliked);
        }

        if cleaned.contains("not") && cleaned.contains("interested") {
            return Some(InteractionName::NotInterested);
        }

        if cleaned.contains("interested") {
            return Some(InteractionName::Interested);
        }

        if cleaned.contains("not") && cleaned.contains("seen") {
            return Some(InteractionName::NotSeen);
        }

        if cleaned.contains("seen") {
            return Some(InteractionName::Seen);
        }

        None
    }
}

pub fn to_all_interaction_names() -> Vec<InteractionName> {
    vec![
        InteractionName::Seen,
        InteractionName::NotSeen,
        InteractionName::Liked,
        InteractionName::Disliked,
        InteractionName::Interested,
        InteractionName::NotInterested,
    ]
}

pub fn to_max_display_string_length() -> usize {
    to_all_interaction_names()
        .iter()
        .map(|name| name.to_display_string().len())
        .max()
        .unwrap_or_default()
}
