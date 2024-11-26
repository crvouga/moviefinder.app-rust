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
            Elem::Element {
                tag_name,
                attrs_safe,
                attrs_unsafe,
                children,
            } => {
                let attrs_safe_str = render_attrs_safe(attrs_safe);
                let attrs_unsafe_str = render_attrs_unsafe(attrs_unsafe);
                let attrs = format!("{}{}", attrs_safe_str, attrs_unsafe_str);
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
            Elem::Fragment(children) => render_children(children, indent_level),
            Elem::Safe(content) => format!("{}{}\n", indent, escape(content)),
            Elem::Unsafe(content) => format!("{}{}\n", indent, content),
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

fn render_attrs_safe(attrs: &HashMap<String, String>) -> String {
    render_attrs(attrs, RenderAttrBehavior::Safe)
}

fn render_attrs_unsafe(attrs: &HashMap<String, String>) -> String {
    render_attrs(attrs, RenderAttrBehavior::Unsafe)
}

enum RenderAttrBehavior {
    Safe,
    Unsafe,
}

fn render_attrs(attrs: &HashMap<String, String>, behavior: RenderAttrBehavior) -> String {
    attrs
        .iter()
        .filter_map(|(name, value)| {
            let name_cleaned = name.trim();
            if name_cleaned.is_empty() {
                None
            } else {
                let value_final = match behavior {
                    RenderAttrBehavior::Safe => escape(value),
                    RenderAttrBehavior::Unsafe => value.to_string(),
                };
                Some(format!(" {}=\"{}\"", name_cleaned, value_final))
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn escape(content: &str) -> String {
    content
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

fn append_doc_type(html: &str) -> String {
    format!("<!DOCTYPE html>\n{}", html)
}
