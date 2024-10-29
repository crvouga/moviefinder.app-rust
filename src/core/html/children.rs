use super::Elem;

impl Elem {
    pub fn child_text(self, value: &str) -> Self {
        self.child(Elem::Safe(value.to_string()))
    }

    pub fn child(mut self, child: Elem) -> Self {
        if let Elem::Element {
            ref mut children, ..
        } = self
        {
            children.push(child);
        }

        self
    }

    pub fn children(mut self, children_new: Vec<Elem>) -> Self {
        if let Elem::Element {
            ref mut children, ..
        } = self
        {
            for child_new in children_new {
                children.push(child_new);
            }
        }

        self
    }
}
