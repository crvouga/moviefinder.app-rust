#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use crate::core::{html::Elem, ui::icon};

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

    pub fn to_name(&self) -> String {
        match self {
            InteractionName::Liked => "Liked".to_string(),
            InteractionName::Disliked => "Disliked".to_string(),
            InteractionName::Interested => "Interested".to_string(),
            InteractionName::NotInterested => "Uninterested".to_string(),
            InteractionName::Seen => "Seen".to_string(),
            InteractionName::NotSeen => "Not Seen".to_string(),
        }
    }
}
