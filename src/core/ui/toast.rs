use super::icon;
use crate::core::{
    html::{button, children::text, div, Elem},
    http::response_writer::ResponseWriter,
};
use std::time::Duration;

#[derive(Default)]
pub enum ToastVariant {
    #[default]
    Dark,
    Success,
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
            duration: Duration::from_secs(3),
            message: "".to_string(),
            variant: ToastVariant::Dark,
        }
    }
}

impl Toast {
    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

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

    pub fn view_root() -> Elem {
        div()
            .id("toast")
            .class("absolute inset-0 pointer-events-none p-6 w-full h-full")
    }

    pub fn view(self) -> Elem {
        let duration_ms = self.duration.as_millis();
        let js_close = "document.getElementById('toast-content').classList.replace('animate-slide-down', 'animate-slide-up')";

        Self::view_root()
        .child(
            div()
            .id("toast-content")
            .class("w-full h-fit p-4 text-lg font-bold pointer-events-auto rounded shadow-xl overflow-hidden animate-slide-down items-center flex z-50")
            .data_on(|b|
                    b.load()
                    .js(&format!("const duration = {}", duration_ms))
                    .js(&js_timeout(self.duration, js_close))
            )
            .class(match self.variant {
                ToastVariant::Success => "bg-green-600 text-white",
                ToastVariant::Dark => "bg-neutral-700 text-white",
                ToastVariant::Error => "border-red-600 border bg-red-800 text-white",
            })
            .child(div().class("flex-1").child(text(&self.message)))
            .child(
                button().aria_label("close toast")
                .on_click(js_close)
                .child(icon::x_mark("size-8"))
            )
        )
    }
}

fn js_timeout(duration: Duration, js: &str) -> String {
    format!("setTimeout(() => {}, {})", js, duration.as_millis())
}

impl ResponseWriter {
    pub async fn send_toast(&mut self, toast: Toast) -> Result<(), std::io::Error> {
        self.send_fragment(toast.view()).await
    }

    pub async fn send_toast_dark(&mut self, message: &str) -> Result<(), std::io::Error> {
        self.send_toast(Toast::dark(message)).await
    }

    pub async fn send_toast_error(&mut self, message: &str) -> Result<(), std::io::Error> {
        self.send_toast(Toast::error(message)).await
    }
}
