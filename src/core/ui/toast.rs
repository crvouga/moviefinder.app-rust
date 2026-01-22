use super::icon;
use crate::core::{
    html::{button, div, unsafe_text, Html},
    http::response_writer::ResponseWriter,
    js::Js,
};
use std::time::Duration;

#[derive(Default)]
pub enum ToastVariant {
    #[default]
    Dark,
    Error,
}

pub struct Toast {
    pub message: String,
    pub variant: ToastVariant,
    pub duration: Duration,
}

impl Default for Toast {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(5),
            message: "".to_string(),
            variant: ToastVariant::Dark,
        }
    }
}

impl Toast {
    pub fn dark(message: &str) -> Self {
        Self {
            message: message.to_string(),
            variant: ToastVariant::Dark,
            ..Default::default()
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            message: message.to_string(),
            variant: ToastVariant::Error,
            ..Default::default()
        }
    }

    pub fn view_root() -> Html {
        div()
            .id("toast")
            .class("absolute inset-0 pointer-events-none p-6 w-full h-full z-50")
    }

    pub fn view(self) -> Html {
        let duration_ms = self.duration.as_millis();
        let js_close = r#"const toastContent = document.getElementById('toast-content'); if (toastContent) { toastContent.classList.replace('animate-slide-down', 'animate-slide-up'); setTimeout(() => { toastContent.remove(); }, 200); }"#;
        let js_close_click = r#"(function() { const toastContent = document.getElementById('toast-content'); if (toastContent) { toastContent.classList.replace('animate-slide-down', 'animate-slide-up'); setTimeout(() => { toastContent.remove(); }, 200); } })()"#;

        Self::view_root()
        .child(
            div()
            .id("toast-content")
            .class("w-full h-fit p-4 text-lg font-bold pointer-events-auto rounded overflow-hidden animate-slide-down items-center flex z-50")
            .class(match self.variant {
                // ToastVariant::Success => "bg-green-600 text-white",
                ToastVariant::Dark => "bg-neutral-700 text-white",
                ToastVariant::Error => "border-red-600 border_  bg-red-800 text-white",
            })
            .child(div().class("flex-1").child(unsafe_text(&self.message.replace("\n", ""))))
            .child(
                button().aria_label("close toast")
                .child(icon::solid::x_mark("size-8"))
            )
        )
    }
}

fn js_timeout(duration: Duration, js: &str) -> String {
    format!("setTimeout(() => {{ {} }}, {})", js, duration.as_millis())
}

impl ResponseWriter {
    pub async fn send_toast(&mut self, _toast: Toast) -> Result<(), crate::core::error::CoreError> {
        // Toasts are disabled.
        // self.send_fragment(toast.view()).await
        Ok(())
    }

    pub async fn send_toast_dark(
        &mut self,
        message: &str,
    ) -> Result<(), crate::core::error::CoreError> {
        self.send_toast(Toast::dark(message)).await
    }

    pub async fn send_toast_error(
        &mut self,
        message: &str,
    ) -> Result<(), crate::core::error::CoreError> {
        self.send_toast(Toast::error(message)).await?;
        self.send_script(&Js::console_error(&Js::quote(message)))
            .await?;
        Ok(())
    }
}
