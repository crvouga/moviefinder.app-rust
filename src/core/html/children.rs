use super::{escape::escape, Elem};

impl Elem {
    pub fn child_text(self, value: &str) -> Self {
        self.child(Elem::Text(escape(value)))
    }

    pub fn child_text_unsafe(self, value: &str) -> Self {
        self.child(Elem::Text(value.to_string()))
    }

    pub fn children(mut self, children: Vec<Elem>) -> Self {
        match self {
            Elem::Tag {
                tag_name: _,
                attrs: _,
                children: ref mut children_prev,
            } => {
                for child_new in children {
                    children_prev.push(child_new);
                }
            }
            Elem::Frag(ref mut children_prev) => {
                for child_new in children {
                    children_prev.push(child_new);
                }
            }
            Elem::Text(_) => (),
        }
        self
    }

    pub fn child(self, child: Elem) -> Self {
        self.children(vec![child])
    }
}
