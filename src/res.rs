use crate::html::Elem;

#[derive(Debug)]
pub enum Res {
    Html(Elem),
    Redirect(String),
}

impl Res {
    pub fn map_html(self, f: impl FnOnce(Elem) -> Elem) -> Res {
        match self {
            Res::Html(body) => Res::Html(f(body)),
            _ => self,
        }
    }
}
