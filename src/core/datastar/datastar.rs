use crate::core::{
    html::Elem,
    http::{request::Request, server_sent_event::ServerSentEvent},
};

pub fn js_get(url: &str) -> String {
    format!("$get('{}')", url)
}

pub fn js_post(url: &str) -> String {
    format!("$post('{}', this)", url)
}

pub fn js_patch(url: &str) -> String {
    format!("$patch('{}', this)", url)
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
        if let Elem::Tag {
            attrs: ref mut attributes,
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

    pub fn data_intersects(mut self, value: &str) -> Self {
        if let Elem::Tag {
            attrs: ref mut attributes,
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
        self.data_intersects(&js_get(url))
    }

    pub fn data_on_then_post(self, event: &str, url: &str) -> Self {
        self.data_on(event, &js_post(url))
    }

    pub fn data_on_then_patch(self, event: &str, url: &str) -> Self {
        self.data_on(event, &js_patch(url))
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

    pub fn data_on_click_push_then_get(self, url: &str) -> Self {
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

impl ServerSentEvent {
    pub fn event_merge_fragments(&mut self) -> &mut Self {
        self.event("datastar-merge-fragments")
    }

    pub fn data_merge_mode(&mut self, mode: &str) -> &mut Self {
        self.data(&format!("mergeMode {}", mode))
    }

    pub fn data_merge_mode_outer(&mut self) -> &mut Self {
        self.data_merge_mode("outer")
    }

    pub fn data_fragments(&mut self, elem: Elem) -> &mut Self {
        let rendered = elem.render();
        let data = format!("fragments {}", rendered.replace("\n", ""));
        self.data(&data)
    }

    pub fn event_execute_script(&mut self) -> &mut Self {
        self.event("datastar-execute-script")
    }

    pub fn data_script(&mut self, script: &str) -> &mut Self {
        let data = format!("script {}", script);
        self.data(&data)
    }

    pub fn data_script_push_url(&mut self, url: &str) -> &mut Self {
        let script: String = format!("window.history.pushState(null, '', '{}');", url);
        self.data_script(&script)
    }

    pub fn data_script_replace_url(&mut self, url: &str) -> &mut Self {
        let script = format!("window.history.replaceState(null, '', '{}');", url);
        self.data_script(&script)
    }
}

impl Request {
    pub fn is_datastar_request(self: &Self) -> bool {
        let fallback = "".to_string();
        let header_value = self.headers.get("datastar-request").unwrap_or(&fallback);
        header_value == "true"
    }
}
