use super::Elem;

impl Elem {
    pub fn child_text(self, value: &str) -> Self {
        self.child(Elem::Safe(value.to_string()))
    }

    pub fn child_unsafe_text(self, value: &str) -> Self {
        self.child(Elem::Unsafe(value.to_string()))
    }

    pub fn child(mut self, child: Elem) -> Self {
        match self {
            Elem::Element {
                tag_name: _,
                attributes: _,
                ref mut children,
            } => {
                children.push(child);
            }
            Elem::Fragment(ref mut children) => {
                children.push(child);
            }
            Elem::Safe(_) | Elem::Unsafe(_) => (),
        }
        self
    }

    pub fn children(mut self, children: Vec<Elem>) -> Self {
        match self {
            Elem::Element {
                tag_name: _,
                attributes: _,
                children: ref mut children_prev,
            } => {
                for child_new in children {
                    children_prev.push(child_new);
                }
            }
            Elem::Fragment(ref mut children_prev) => {
                for child_new in children {
                    children_prev.push(child_new);
                }
            }
            Elem::Safe(_) | Elem::Unsafe(_) => (),
        }
        self
    }
}
