use super::html::Elem;

impl Elem {
    #[allow(dead_code)]
    pub fn src_tailwind_play_cdn(self) -> Self {
        self.src("https://cdn.tailwindcss.com")
    }
}
