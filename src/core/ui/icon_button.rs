use crate::core::html::*;

#[derive(Default)]
pub struct IconButton {
    label: String,
    icon: Option<Box<dyn FnOnce(String) -> Html>>,
    id: Option<String>,
    bind_disabled: Option<String>,
}

impl IconButton {
    // pub fn id(mut self, id: &str) -> Self {
    //     self.id = Some(id.to_string());
    //     self
    // }

    pub fn icon(mut self, icon: impl FnOnce(String) -> Html + 'static) -> Self {
        self.icon = Some(Box::new(icon));
        self
    }

    pub fn bind_disabled(mut self, value: String) -> Self {
        self.bind_disabled = Some(value);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn view(self) -> Html {
        let id = self.id.unwrap_or_default();
        let icon = if let Some(icon) = self.icon {
            icon("size-8".to_owned())
        } else {
            Html::default()
        };

        button()
            .id(&id)
            .class("group relative flex items-center p-3 text-lg font-bold text-white rounded-full")
            .class("enabled:active:opacity-active")
            .class("disabled:opacity-40 disabled:cursor-not-allowed")
            .class("rounded bg-transparent")
            .type_button()
            .map(|e| match self.bind_disabled {
                Some(bind_disabled) => e.data_attributes("disabled", &bind_disabled),
                None => e,
            })
            .child(icon)
            .aria_label(&self.label)
    }
}
