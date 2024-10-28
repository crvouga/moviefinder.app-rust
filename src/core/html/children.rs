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
                children: existing_children,
            } => {
                let mut new_children = existing_children.clone();
                for child_new in children {
                    new_children.push(child_new.clone())
                }
                Elem::Element {
                    tag_name: tag_name.clone(),
                    attributes: attributes.clone(),
                    children: new_children,
                }
            }
            _ => self.clone(),
        }
    }
}
