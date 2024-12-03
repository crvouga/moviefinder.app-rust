use serde::{Deserialize, Serialize};

use crate::media::interaction::interaction_::MediaInteraction;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Record(MediaInteraction),
}
