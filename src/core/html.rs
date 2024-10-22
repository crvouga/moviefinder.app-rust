#[derive(Clone, Debug)]
pub struct Attr {
    name: String,
    value: String,
}

#[derive(Clone, Debug)]
pub enum Elem {
    Element {
        tag_name: String,
        attributes: Vec<Attr>,
        children: Vec<Elem>,
    },
    Fragment(Vec<Elem>),
    Safe(String),
    Unsafe(String),
}

fn append_doc_type(html: &str) -> String {
    format!("<!DOCTYPE html>\n{}", html)
}

impl Elem {
    pub fn render(&self) -> String {
        append_doc_type(&self.render_with_indent(0))
    }

    pub fn render_with_indent(&self, indent_level: usize) -> String {
        let indent = "\t".repeat(indent_level);
        match self {
            Elem::Element {
                tag_name,
                attributes,
                children,
            } => {
                let attrs = render_attrs(attributes);
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
            Elem::Safe(content) => format!("{}{}\n", indent, escape_html(content)),
            Elem::Unsafe(content) => format!("{}{}\n", indent, content),
        }
    }
}
pub fn render_children(children: &[Elem], indent_level: usize) -> String {
    children
        .iter()
        .map(|child| child.render_with_indent(indent_level))
        .collect::<Vec<String>>()
        .join("")
}

pub fn render_attrs(attrs: &[Attr]) -> String {
    attrs
        .iter()
        .map(|attr| format!(" {}=\"{}\"", attr.name, attr.value))
        .collect::<Vec<String>>()
        .join("")
}

fn escape_html(content: &str) -> String {
    content
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

pub fn unsafe_html(content: &str) -> Elem {
    Elem::Unsafe(content.to_string())
}

pub fn fragment(children: &[Elem]) -> Elem {
    Elem::Fragment(children.to_vec())
}

pub fn attr(name: &str, value: &str) -> Attr {
    Attr {
        name: name.to_string(),
        value: value.to_string(),
    }
}

pub fn charset(value: &str) -> Attr {
    attr("charset", value)
}

pub fn lang(value: &str) -> Attr {
    attr("lang", value)
}

pub fn text(value: &str) -> Elem {
    Elem::Safe(value.to_string())
}

pub fn name(value: &str) -> Attr {
    attr("name", value)
}

pub fn content(value: &str) -> Attr {
    attr("content", value)
}

pub fn src(value: &str) -> Attr {
    attr("src", value)
}

pub fn defer(value: &str) -> Attr {
    attr("defer", value)
}

pub fn class(value: &str) -> Attr {
    attr("class", value)
}

pub fn class_list(values: &[&str]) -> Attr {
    attr("class", &values.join(" "))
}

pub fn aria_label(value: &str) -> Attr {
    attr("aria-label", value)
}

pub fn id(value: &str) -> Attr {
    attr("id", value)
}

pub fn rel(value: &str) -> Attr {
    attr("rel", value)
}

pub fn href(value: &str) -> Attr {
    attr("href", value)
}

pub fn width(value: &str) -> Attr {
    attr("width", value)
}

pub fn height(value: &str) -> Attr {
    attr("height", value)
}

pub fn elem(tag_name: &str, attributes: &[Attr], children: &[Elem]) -> Elem {
    Elem::Element {
        tag_name: tag_name.to_string(),
        attributes: attributes.to_vec(),
        children: children.to_vec(),
    }
}

pub fn void_element(tag_name: &str, attributes: &[Attr]) -> Elem {
    Elem::Element {
        tag_name: tag_name.to_string(),
        attributes: attributes.to_vec(),
        children: vec![],
    }
}

pub fn meta(attrs: &[Attr]) -> Elem {
    void_element("meta", attrs)
}

pub fn title(value: &str) -> Elem {
    elem("title", &[], &[text(value)])
}

pub fn link(attrs: &[Attr]) -> Elem {
    void_element("link", attrs)
}

pub fn script(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("script", attrs, children)
}

pub fn div(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("div", attrs, children)
}

pub fn p(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("p", attrs, children)
}

pub fn button(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("button", attrs, children)
}

pub fn html(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("html", attrs, children)
}

pub fn head(children: &[Elem]) -> Elem {
    elem("head", &[], children)
}

pub fn body(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("body", attrs, children)
}

pub fn a(attrs: &[Attr], children: &[Elem]) -> Elem {
    elem("a", attrs, &children)
}
