use crate::core::html::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Chip {
    pub id: String,
    pub label: String,
    pub name: String,
    pub checked: bool,
    pub disabled: bool,
    pub size: Size,
}

impl Chip {
    pub fn new() -> Self {
        Chip::default()
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn view(self) -> Elem {
        div()
        .child(
            input()
            .class("hidden peer")
            .type_("checkbox")
            .id(&self.id)
            .name(&self.name)
            .value(&self.id)
            .checked(self.checked)
            .disabled(self.disabled)
        )
        .child(
            label()
            .for_(&self.id)
            .class("flex items-center justify-center rounded-full font-bold w-fit bg-neutral-800 text-white cursor-pointer select-none")
            .class("peer-checked:bg-white peer-checked:font-bold peer-checked:text-black enabled:active:opacity-80")
            .class(&self.size.to_class())
            .child_text(&self.label)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    fn to_class(self) -> String {
        match self {
            Size::Small => "text-xs px-2 py-1".to_string(),
            Size::Medium => "text-sm px-2.5 py-1.5".to_string(),
            Size::Large => "text-base px-3 py-2".to_string(),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::Medium
    }
}
