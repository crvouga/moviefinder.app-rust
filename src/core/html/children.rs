use super::{escape::escape, Elem};

pub fn text(value: &str) -> Elem {
    Elem::Text(escape(value))
}

pub fn text_unsafe(value: &str) -> Elem {
    Elem::Text(value.to_string())
}

impl Elem {
    pub fn child_text(self, value: &str) -> Self {
        self.child(text(value))
    }

    pub fn child_text_unsafe(self, value: &str) -> Self {
        self.child(text_unsafe(value))
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
