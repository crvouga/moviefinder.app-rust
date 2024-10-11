pub use Attr::*;
pub use Elem::*;

#[derive(Clone, Debug)]
pub enum Attr {
    Charset(String),
    Name(String),
    Content(String),
    Src(String),
    Class(String),
    Id(String),
    Rel(String),
    Href(String),
}

#[derive(Clone, Debug)]
pub enum Elem {
    Meta(Vec<Attr>),
    Link(Vec<Attr>),
    Script(Vec<Attr>, String),
    Div(Vec<Attr>, Vec<Elem>),
    Html(Vec<Elem>),
    Head(Vec<Elem>),
    Body(Vec<Attr>, Vec<Elem>),
    Text(String),
}

impl Elem {
    pub fn render(&self) -> String {
        match self {
            Meta(attrs) => format!("<meta {} />", render_attributes(attrs)),
            Link(attrs) => format!("<link {} />", render_attributes(attrs)),
            Script(attrs, content) => {
                format!("<script {}>{}</script>", render_attributes(attrs), content)
            }
            Div(attrs, children) => format!(
                "<div {}>{}</div>",
                render_attributes(attrs),
                render_children(children)
            ),
            Html(children) => format!("<!DOCTYPE html><html>{}</html>", render_children(children)),
            Head(children) => format!("<head>{}</head>", render_children(children)),
            Body(attrs, children) => format!(
                "<body {}>{}</body>",
                render_attributes(attrs),
                render_children(children)
            ),
            Text(value) => value.to_string(),
        }
    }
}

pub fn meta(attrs: &[Attr]) -> Elem {
    Meta(attrs.to_vec())
}

pub fn link(attrs: &[Attr]) -> Elem {
    Link(attrs.to_vec())
}

pub fn script(attrs: &[Attr], content: &str) -> Elem {
    Script(attrs.to_vec(), content.to_string())
}

pub fn div(attrs: &[Attr], children: &[Elem]) -> Elem {
    Div(attrs.to_vec(), children.to_vec())
}

pub fn html(children: &[Elem]) -> Elem {
    Html(children.to_vec())
}

pub fn head(children: &[Elem]) -> Elem {
    Head(children.to_vec())
}

pub fn body(attrs: &[Attr], children: &[Elem]) -> Elem {
    Body(attrs.to_vec(), children.to_vec())
}

pub fn charset(value: &str) -> Attr {
    Charset(value.to_string())
}

pub fn name(value: &str) -> Attr {
    Name(value.to_string())
}

pub fn content(value: &str) -> Attr {
    Content(value.to_string())
}

pub fn src(value: &str) -> Attr {
    Src(value.to_string())
}

pub fn class(value: &str) -> Attr {
    Class(value.to_string())
}

pub fn id(value: &str) -> Attr {
    Id(value.to_string())
}

pub fn rel(value: &str) -> Attr {
    Rel(value.to_string())
}

pub fn href(value: &str) -> Attr {
    Href(value.to_string())
}

pub fn text(value: &str) -> Elem {
    Text(value.to_string())
}

pub fn render_attributes(attrs: &[Attr]) -> String {
    attrs
        .iter()
        .map(|attr| match attr {
            Charset(value) => format!("charset=\"{}\"", value),
            Name(value) => format!("name=\"{}\"", value),
            Content(value) => format!("content=\"{}\"", value),
            Src(value) => format!("src=\"{}\"", value),
            Class(value) => format!("class=\"{}\"", value),
            Id(value) => format!("id=\"{}\"", value),
            Rel(value) => format!("rel=\"{}\"", value),
            Href(value) => format!("href=\"{}\"", value),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn render_children(children: &[Elem]) -> String {
    children
        .iter()
        .map(|child| child.render())
        .collect::<Vec<String>>()
        .join("")
}
