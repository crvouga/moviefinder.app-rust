use super::Elem;

impl Elem {
    pub fn child_text(self, value: &str) -> Self {
        self.child(Elem::Safe(value.to_string()))
    }

    pub fn child(mut self, child_new: Elem) -> Self {
        match self {
            Elem::Element {
                tag_name: _,
                attributes: _,
                ref mut children,
            } => {
                children.push(child_new);
            }
            Elem::Fragment(ref mut children) => {
                children.push(child_new);
            }
            Elem::Safe(_) | Elem::Unsafe(_) => (),
        }
        self
    }

    pub fn children(mut self, children_new: Vec<Elem>) -> Self {
        match self {
            Elem::Element {
                tag_name: _,
                attributes: _,
                ref mut children,
            } => {
                for child_new in children_new {
                    children.push(child_new);
                }
            }
            Elem::Fragment(ref mut children) => {
                for child_new in children_new {
                    children.push(child_new);
                }
            }
            Elem::Safe(_) | Elem::Unsafe(_) => (),
        }
        self
    }
}
