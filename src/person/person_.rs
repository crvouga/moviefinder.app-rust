use serde::{Deserialize, Serialize};

use crate::core::image_set::ImageSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Person {
    pub name: String,
    pub id: String,
    pub profile: ImageSet,
}
