use crate::core::html::*;

#[derive(Default)]
pub struct IconButton {
    label: String,
    icon: Option<Box<dyn Fn(String) -> Elem>>,
    id: Option<String>,
}

impl IconButton {
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn icon(mut self, icon: impl Fn(String) -> Elem + 'static) -> Self {
        self.icon = Some(Box::new(icon));
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn view(self) -> Elem {
        let id = self.id.unwrap_or_default();
        let icon = if let Some(icon) = self.icon {
            icon("size-8".to_owned())
        } else {
            Elem::default()
        };

        button()
            .id(&id)
            .class("group relative flex items-center p-3 text-lg font-bold text-white rounded-full")
            .class("active:opacity-60")
            .class("rounded bg-transparent")
            .type_("button")
            .child(div().class("flex items-center justify-center").child(icon))
            .aria_label(&self.label)
    }
}
