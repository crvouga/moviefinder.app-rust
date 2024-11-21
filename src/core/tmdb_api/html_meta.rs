use crate::core::html::{link, Elem};

impl Elem {
    pub fn meta_tmdb_api(self) -> Self {
        self.child(link().rel("preconnect").href("https://image.tmdb.org"))
    }
}
