use std::collections::HashMap;

use super::Elem;

impl Elem {
    pub fn render(&self) -> String {
        self.render_with_indent(0)
        // append_doc_type(&self.render_with_indent(0))
    }

    pub fn render_with_indent(&self, indent_level: usize) -> String {
        let indent = "\t".repeat(indent_level);
        match self {
            Elem::Tag {
                tag_name,
                attrs,
                children,
            } => {
                let attrs = render_attrs(attrs);
                if children.is_empty() {
                    format!("{}<{}{}></{}>\n", indent, tag_name, attrs, tag_name)
                } else {
                    let children_rendered = render_children(children, indent_level + 1);
                    format!(
                        "{}<{}{}>\n{}{}</{}>\n",
                        indent, tag_name, attrs, children_rendered, indent, tag_name
                    )
                }
            }
            Elem::Frag(children) => render_children(children, indent_level),
            Elem::Text(content) => format!("{}{}\n", indent, content),
        }
    }
}

fn render_children(children: &[Elem], indent_level: usize) -> String {
    children
        .iter()
        .map(|child| child.render_with_indent(indent_level))
        .collect::<Vec<String>>()
        .join("")
}

fn render_attrs(attrs: &HashMap<String, String>) -> String {
    attrs
        .iter()
        .filter_map(|(name, value)| {
            let name_cleaned = name.trim();
            if name_cleaned.is_empty() {
                None
            } else {
                Some(format!(" {}=\"{}\"", name_cleaned, value))
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn append_doc_type(html: &str) -> String {
    format!("<!DOCTYPE html>\n{}", html)
}
