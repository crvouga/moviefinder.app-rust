use super::{escape::escape, Html};

pub fn text(value: &str) -> Html {
    Html::Text(escape(value))
}

pub fn text_unsafe(value: &str) -> Html {
    Html::Text(value.to_string())
}

impl Html {
    pub fn child_text(self, value: &str) -> Self {
        self.child(text(value))
    }

    pub fn child_text_unsafe(self, value: &str) -> Self {
        self.child(text_unsafe(value))
    }

    pub fn children(mut self, children: Vec<Html>) -> Self {
        match self {
            Html::Tag {
                tag_name: _,
                attrs: _,
                children: ref mut children_prev,
            } => {
                for child_new in children {
                    children_prev.push(child_new);
                }
            }
            Html::Frag(ref mut children_prev) => {
                for child_new in children {
                    children_prev.push(child_new);
                }
            }
            Html::Text(_) => (),
        }
        self
    }

    pub fn child(self, child: Html) -> Self {
        self.children(vec![child])
    }
}
