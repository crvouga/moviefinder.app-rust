use super::alert;
use crate::html::*;

pub fn page(label: &str) -> Elem {
    div(
        &[class("flex h-full w-full items-center justify-center p-8")],
        &[alert::error(label)],
    )
}
