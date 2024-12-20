use crate::core::html::*;

#[derive(Default)]
pub struct ButtonGroup {
    orientation: ButtonGroupOrientation,
}

#[derive(Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    // Vertical,
}

impl ButtonGroup {
    // pub fn orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
    //     self.orientation = orientation;
    //     self
    // }
    // pub fn orientation_vertical(self) -> Self {
    //     self.orientation(ButtonGroupOrientation::Vertical)
    // }
    pub fn view(self) -> Elem {
        div()
            .class("flex items-center justify-center w-full border-t")
            .class(match self.orientation {
                ButtonGroupOrientation::Horizontal => "flex-row min-h-bar",
                // ButtonGroupOrientation::Vertical => "flex-col",
            })
    }
}

#[derive(Default)]
pub struct ButtonGroupMember {
    text: String,
    icon: Option<Elem>,
    active: bool,
    disabled: bool,
    id: String,
}

impl ButtonGroupMember {
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    // pub fn id(mut self, id: &str) -> Self {
    //     self.id = id.to_string();
    //     self
    // }

    // pub fn disabled(mut self, disabled: bool) -> Self {
    //     self.disabled = disabled;
    //     self
    // }

    pub fn icon(mut self, icon: Elem) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn view(&self) -> Elem {
        button()
        .class_list(
            &[
                    "flex flex-1 items-center justify-center gap-0.5 flex-col text-xs h-full cursor-pointer select-none active:opacity-75",
                    if self.active && !self.disabled {
                        "text-blue-500"
                    } else {
                        ""
                    },
                    if self.disabled {
                        "opacity-30 pointer-events-none cursor-not-allowed"
                    } else {
                        ""
                    },
                ]
        )
        .disabled(self.disabled)
        .child(
            self.icon.clone().unwrap_or_else(|| frag())
        ).child_text(
            &self.text,
        )
        .id(&self.id)
    }
}
