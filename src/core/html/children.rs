use super::Elem;

impl Elem {
    pub fn child_text(self, value: &str) -> Self {
        self.child(Elem::Safe(value.to_string()))
    }

    pub fn child(self, child: Elem) -> Self {
        self.children(&[child])
    }

    pub fn children(mut self, children_new: &[Elem]) -> Self {
        if let Elem::Element {
            ref mut children, ..
        } = self
        {
            children.extend_from_slice(children_new);
        }

        self
    }
}
