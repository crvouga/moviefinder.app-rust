use crate::core::html::{pre, unsafe_text, Elem};

impl Elem {
    #[allow(dead_code)]
    pub fn debug<T>(self, value: T, on: bool) -> Self
    where
        T: serde::Serialize,
    {
        if on {
            self.child(pre().child(unsafe_text(
                &serde_json::to_string_pretty(&value).unwrap_or("".to_owned()),
            )))
        } else {
            self
        }
    }
}
