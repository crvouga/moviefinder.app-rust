use crate::html::*;

const SPINNER_HTML: &'static str = r#"
<svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" stroke="currentColor" viewBox="0 0 24 24" ATTRS>
    <path
        fill-rule="evenodd"
        d="M12 19a7 7 0 100-14 7 7 0 000 14zm0 3c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"
        clip-rule="evenodd"
        opacity="0.2"
    />
    <path d="M2 12C2 6.477 6.477 2 12 2v3a7 7 0 00-7 7H2z" />
</svg>
"#;

pub fn spinner(attrs: &[Attr]) -> Elem {
    let replaced: String = SPINNER_HTML.replace("ATTRS", &render_attrs(&attrs));
    return unsafe_html(&replaced);
}
