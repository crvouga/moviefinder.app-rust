use crate::core::{html::Elem, http::response_writer::HttpResponseWriter};

pub fn js_get(url: &str) -> String {
    format!("$get('{}')", url)
}

pub fn js_post(url: &str) -> String {
    format!("$post('{}', this)", url)
}

impl Elem {
    pub fn src_datastar(self) -> Self {
        self.src("https://cdn.jsdelivr.net/gh/starfederation/datastar/bundles/datastar.js")
    }

    pub fn data_model(self, value: &str) -> Self {
        self.attr_unsafe("data-model", value)
    }

    pub fn data_store(self, value: &str) -> Self {
        self.attr_unsafe("data-store", value)
    }

    pub fn data_text(self, value: &str) -> Self {
        self.attr_unsafe("data-text", value)
    }

    pub fn data_on(mut self, event: &str, value: &str) -> Self {
        if let Elem::Element {
            attrs_unsafe: ref mut attributes,
            ..
        } = self
        {
            let key = &format!("data-on-{}", event);
            let existing = attributes.get(key).map_or("", |attr| attr.as_str());

            let new = if existing.is_empty() {
                value.trim().to_string()
            } else {
                format!("{}; {}", existing, value).trim().to_string()
            };

            attributes.insert(key.to_string(), new);
        }

        self
    }

    pub fn data_intersects(mut self, event: &str, value: &str) -> Self {
        if let Elem::Element {
            attrs_unsafe: ref mut attributes,
            ..
        } = self
        {
            let key = "data-intersects";
            let existing = attributes.get(key).map_or("", |attr| attr.as_str());

            let new = if existing.is_empty() {
                value.trim().to_string()
            } else {
                format!("{}; {}", existing, value).trim().to_string()
            };

            attributes.insert(key.to_string(), new);
        }

        self
    }

    pub fn data_intersects_get(self, url: &str) -> Self {
        self.data_intersects("get", &js_get(url))
    }

    pub fn data_on_then_post(self, event: &str, url: &str) -> Self {
        self.data_on(event, &js_post(url))
    }

    pub fn data_on_click(self, value: &str) -> Self {
        self.data_on("click", value)
    }

    pub fn data_on_load(self, value: &str) -> Self {
        self.data_on("load", value)
    }

    pub fn data_on_click_get(self, url: &str) -> Self {
        self.data_on_click(&js_get(url))
    }

    pub fn data_on_click_push_get(self, url: &str) -> Self {
        let push_url_script = format!("window.history.pushState(null, '', '{}')", url);
        let get_script = js_get(url);
        let script = format!("{}; {}", push_url_script, get_script);
        self.data_on_click(&script)
    }

    pub fn data_on_load_get(self, url: &str) -> Self {
        self.data_on_load(&js_get(url))
    }

    pub fn data_on_click_post(self, url: &str) -> Self {
        self.data_on_click(&format!("$post('{}', this)", url))
    }
}

fn fragments(elem: Elem) -> String {
    let rendered = elem.render();
    format!("fragments {}\n\n", rendered.replace("\n", ""))
}

fn script(script: &str) -> String {
    format!("script {}\n\n", script)
}

impl HttpResponseWriter {
    pub async fn merge_fragment(&mut self, elem: Elem) {
        let _ = self
            .write_sse_event("datastar-merge-fragments", vec![&fragments(elem)])
            .await;
    }

    pub async fn execute_script(&mut self, script_str: &str) {
        let _ = self
            .write_sse_event("datastar-execute-script", vec![&script(script_str)])
            .await;
    }

    pub async fn execute_script_push_url(&mut self, url: &str) {
        let push_url_script = format!("window.history.pushState(null, '', '{}');", url);
        self.execute_script(&push_url_script).await;
    }

    pub async fn execute_script_replace_url(&mut self, url: &str) {
        let replace_url_script = format!("window.history.replaceState(null, '', '{}');", url);
        self.execute_script(&replace_url_script).await;
    }
}
