use super::Elem;

impl Elem {
    pub fn child_text(&self, value: &str) -> Elem {
        self.child(Elem::Safe(value.to_string()))
    }

    pub fn child(&self, child: Elem) -> Elem {
        self.children(&[child])
    }

    pub fn children(&self, children: &[Elem]) -> Elem {
        match self {
            Elem::Element {
                tag_name,
                attributes,
                children: _,
            } => {
                let mut new_children = children.to_vec();
                new_children.extend_from_slice(children);
                Elem::Element {
                    tag_name: tag_name.clone(),
                    attributes: attributes.clone(),
                    children: new_children.clone(),
                }
            }
            _ => self.clone(),
        }
    }
}
