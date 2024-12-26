use super::html::Html;

impl Html {
    #[allow(dead_code)]
    pub fn src_tailwind_play_cdn(self) -> Self {
        self.src("https://cdn.tailwindcss.com")
    }
}
