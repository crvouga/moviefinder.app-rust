use crate::media::media_id::MediaId;
use serde::{Deserialize, Serialize};

use super::interaction_form_::InteractionForm;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InteractionFormOrientation {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InteractionFormViewConfig {
    pub namespace: String,
    pub orientation: InteractionFormOrientation,
    pub media_id: MediaId,
    pub form: Option<InteractionForm>,
}

impl InteractionFormViewConfig {
    pub fn namespace(mut self, namespace: String) -> Self {
        self.namespace = namespace;
        self
    }

    pub fn orientation(mut self, orientation: InteractionFormOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn orientation_vertical(self) -> Self {
        self.orientation(InteractionFormOrientation::Vertical)
    }

    pub fn orientation_horizontal(self) -> Self {
        self.orientation(InteractionFormOrientation::Horizontal)
    }

    pub fn media_id(mut self, media_id: MediaId) -> Self {
        self.media_id = media_id;
        self
    }

    pub fn form(mut self, form: InteractionForm) -> Self {
        self.form = Some(form);
        self
    }
}
