fn fallback_empty_string(value: &str) -> String {
    if value.trim().is_empty() {
        "''".to_string()
    } else {
        value.to_string()
    }
}

pub struct Js {}

impl Js {
    pub fn empty_string() -> String {
        "''".to_string()
    }

    // pub fn dot_length(value: &str) -> String {
    //     format!("{}.length", value)
    // }

    // pub fn eq(left: &str, right: &str) -> String {
    //     format!("{} === {}", left, right)
    // }

    pub fn assign(variable: &str, value: &str) -> String {
        let value_final = fallback_empty_string(value);
        format!("{} = {}", variable, value_final)
    }

    pub fn not(value: &str) -> String {
        format!("!({})", value)
    }

    pub fn is_str(value: &str) -> String {
        format!("(typeof ({}) === 'string')", value)
    }

    pub fn replace_url(url: &str) -> String {
        format!("window.history.replaceState(null, '', '{}')", url)
    }

    pub fn push_url(url: &str) -> String {
        format!("window.history.pushState(null, '', '{}')", url)
    }

    pub fn focus(selector: &str) -> String {
        format!("document.querySelector('{}').focus()", selector)
    }

    pub fn console_log(message: &str) -> String {
        format!("console.log('{}')", message)
    }

    pub fn console_error(message: &str) -> String {
        format!("console.error('{}')", message)
    }

    pub fn str(value: &str) -> String {
        format!("'{}'", value)
    }
}
