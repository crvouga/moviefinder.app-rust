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

const HOME_HTML: &'static str = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6" ATTRS>
  <path d="M11.47 3.841a.75.75 0 0 1 1.06 0l8.69 8.69a.75.75 0 1 0 1.06-1.061l-8.689-8.69a2.25 2.25 0 0 0-3.182 0l-8.69 8.69a.75.75 0 1 0 1.061 1.06l8.69-8.689Z" />
  <path d="m12 5.432 8.159 8.159c.03.03.06.058.091.086v6.198c0 1.035-.84 1.875-1.875 1.875H15a.75.75 0 0 1-.75-.75v-4.5a.75.75 0 0 0-.75-.75h-3a.75.75 0 0 0-.75.75V21a.75.75 0 0 1-.75.75H5.625a1.875 1.875 0 0 1-1.875-1.875v-6.198a2.29 2.29 0 0 0 .091-.086L12 5.432Z" />
</svg>
"#;

pub fn home(attrs: &[Attr]) -> Elem {
    let replaced: String = HOME_HTML.replace("ATTRS", &render_attrs(&attrs));
    return unsafe_html(&replaced);
}

const USER_CIRCLE_HTML: &'static str = r#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
  <path fill-rule="evenodd" d="M18.685 19.097A9.723 9.723 0 0 0 21.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 0 0 3.065 7.097A9.716 9.716 0 0 0 12 21.75a9.716 9.716 0 0 0 6.685-2.653Zm-12.54-1.285A7.486 7.486 0 0 1 12 15a7.486 7.486 0 0 1 5.855 2.812A8.224 8.224 0 0 1 12 20.25a8.224 8.224 0 0 1-5.855-2.438ZM15.75 9a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" clip-rule="evenodd" />
</svg>
"#;

pub fn user_circle(attrs: &[Attr]) -> Elem {
    return unsafe_html(USER_CIRCLE_HTML);
}
