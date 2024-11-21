use super::html::Elem;

impl Elem {
    pub fn src_tailwindcss(self) -> Self {
        self.src("https://cdn.tailwindcss.com")
    }
}
