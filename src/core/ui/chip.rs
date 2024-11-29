use crate::core::html::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Chip {
    pub id: String,
    pub label: String,
    pub name: String,
    pub checked: bool,
    pub disabled: bool,
    pub size: ChipSize,
    pub image: Option<String>,
    pub input_model: String,
    pub bind_checked: String,
}

impl Chip {
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn input_model(mut self, input_model: &str) -> Self {
        self.input_model = input_model.to_string();
        self
    }

    pub fn image(mut self, image: &str) -> Self {
        self.image = Some(image.to_string());
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

    pub fn bind_checked(mut self, value: &str) -> Self {
        self.bind_checked = value.to_string();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn size(mut self, size: ChipSize) -> Self {
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
            .on_click("event.stopPropagation()")
            .map(|e| {
                if self.bind_checked.len() > 0 {
                    e.data_bind("checked", &self.bind_checked)
                } else {
                    e
                }
            })
        )
        .child(
            label()
            .for_(&self.id)
            .class("flex items-center justify-center rounded-full font-bold w-fit bg-neutral-800 border border-neutral-800 text-white cursor-pointer select-none truncate whitespace-nowrap")
            .class("peer-checked:bg-white peer-checked:font-bold peer-checked:text-black enabled:active:opacity-80")
            .class(&self.size.to_text_size())
            .class(&self.size.to_h())
            .tab_index(0)
            .map(|e| {
                match self.image {
                    None => e,
                    Some(image_src) => if image_src.trim().is_empty() {
                        e
                    } else {
                        e.child(
                            img()
                            .class(&self.size.to_h())
                            .class("shrink-0 aspect-square object-cover rounded-full overflow-hidden bg-neutral-700 border-neutral-800 pointer-events-none")
                            .src(&image_src)
                        )
                    }
                }
            })
            .child(
                div()
                .class(&self.size.to_py())
                .class(&self.size.to_px())
                .child_text(&self.label)
            )
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ChipSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ChipSize {
    fn to_text_size(&self) -> String {
        match self {
            ChipSize::Small => "text-sm".to_string(),
            ChipSize::Medium => "text-base".to_string(),
            ChipSize::Large => "text-lg".to_string(),
        }
    }

    fn to_h(&self) -> String {
        match self {
            ChipSize::Small => "h-9".to_string(),
            ChipSize::Medium => "h-10".to_string(),
            ChipSize::Large => "h-12".to_string(),
        }
    }

    fn to_px(&self) -> String {
        match self {
            ChipSize::Small => "px-2".to_string(),
            ChipSize::Medium => "px-2.5".to_string(),
            ChipSize::Large => "px-3.5".to_string(),
        }
    }
    fn to_py(&self) -> String {
        match self {
            ChipSize::Small => "py-1".to_string(),
            ChipSize::Medium => "py-1.5".to_string(),
            ChipSize::Large => "py-2.5".to_string(),
        }
    }
}
