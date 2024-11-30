use std::time::Duration;

use crate::core::{
    html::{code, pre, Elem},
    http::{json_data::JsonData, request::Request, server_sent_event::ServerSentEvent},
    params::{Params, ParamsHashMap},
    url_encoded,
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

    pub fn data_ref(self, value: &str) -> Self {
        self.attr_unsafe("data-ref", value)
    }

    pub fn data_store(self, value: &str) -> Self {
        self.attr_unsafe("data-store", value)
    }

    pub fn data_persist(self, value: &str) -> Self {
        self.attr_unsafe("data-persist", value)
    }

    pub fn data_on_store_change(mut self, value: &str) -> Self {
        if let Elem::Tag {
            attrs: ref mut attributes,
            ..
        } = self
        {
            let key = "data-on-store-change";
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

    pub fn data_class(self, value: &str) -> Self {
        self.attr_unsafe("data-class", value)
    }

    pub fn data_on_store_change_patch(self, url: &str) -> Self {
        self.data_on_store_change(&js_patch(url))
    }

    pub fn data_text(self, value: &str) -> Self {
        self.attr_unsafe("data-text", value)
    }

    pub fn data_indicator(self, value: &str) -> Self {
        self.attr_unsafe("data-indicator", value)
    }

    pub fn data_bind(self, attr: &str, value: &str) -> Self {
        self.attr_unsafe(&format!("data-bind-{}", attr), value)
    }

    pub fn data_show(self, value: &str) -> Self {
        self.attr_unsafe("data-show", value)
    }

    pub fn data_computed(self, name: &str, value: &str) -> Self {
        self.attr_unsafe(&format!("data-computed-{}", name), value)
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

    pub fn data_on_input(self, value: &str) -> Self {
        self.data_on("input", value)
    }

    pub fn data_on_input_get(self, url: &str) -> Self {
        self.data_on_input(&js_get(url))
    }

    pub fn data_on_input_debounce(self, duration: Duration, value: &str) -> Self {
        let ms = duration.as_millis();

        self.data_on(&format!("input.debounce_{}ms", ms), value)
    }

    pub fn data_on_input_debounce_get(self, duration: Duration, url: &str) -> Self {
        self.data_on_input_debounce(duration, &js_get(url))
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

    pub fn data_on_patch(self, event: &str, url: &str) -> Self {
        self.data_on(event, &&js_patch(url))
    }

    pub fn data_on_load_get(self, url: &str) -> Self {
        self.data_on_load(&js_get(url))
    }

    pub fn data_on_click_post(self, url: &str) -> Self {
        self.data_on_click(&format!("$post('{}', this)", url))
    }

    pub fn data_on_click_patch(self, url: &str) -> Self {
        self.data_on_click(&js_patch(url))
    }

    pub fn child_debug_store(self) -> Self {
        self.child(code().child(pre().data_text("JSON.stringify(ctx.store(), null, 2)")))
    }
}

impl ServerSentEvent {
    pub fn event_merge_fragments(&mut self) -> &mut Self {
        self.event("datastar-merge-fragments")
    }

    pub fn event_merge_signals(&mut self) -> &mut Self {
        self.event("datastar-merge-signals")
    }

    pub fn data_signals(&mut self, value: &str) -> &mut Self {
        self.data(&format!("signals {}", value))
    }

    pub fn data_only_if_missing(&mut self, value: bool) -> &mut Self {
        if value {
            self.data("onlyIfMissing true")
        } else {
            self.data("onlyIfMissing false")
        }
    }

    pub fn data_merge_mode(&mut self, mode: &str) -> &mut Self {
        self.data(&format!("mergeMode {}", mode))
    }

    pub fn data_merge_mode_outer(&mut self) -> &mut Self {
        self.data_merge_mode("outer")
    }

    pub fn data_merge_mode_before(&mut self) -> &mut Self {
        self.data_merge_mode("before")
    }

    pub fn data_selector(&mut self, selector: &str) -> &mut Self {
        self.data(&format!("selector {}", selector))
    }

    pub fn data_selector_id(&mut self, id: &str) -> &mut Self {
        self.data_selector(&format!("#{}", id))
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

    pub fn data_script_redirect(&mut self, url: &str) -> &mut Self {
        let script = format!("window.location = '{}'", url);
        self.data_script(&script)
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

    pub fn datastar_params(self: &Self) -> ParamsHashMap {
        let datastar_params = self.url.query_params.get_first("datastar");

        if let Some(urlencoded_json) = datastar_params {
            let json = url_encoded::decode(urlencoded_json);
            JsonData::from_string(&json).params
        } else {
            self.params()
        }
    }
}
