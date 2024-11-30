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

#[derive(Debug, Clone, Default)]
pub struct Builder {}

#[derive(Debug, Clone)]
pub enum BuilderVariant {
    On(BuilderOn),
    Intersects(BuilderIntersects),
}

#[derive(Debug, Clone, Default)]
pub struct BuilderOn {
    event: String,
    modifiers: Vec<String>,
    actions: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BuilderIntersects {
    event: String,
    modifiers: Vec<String>,
    actions: Vec<String>,
}

pub trait BuilderShared {
    fn to_attr_key_string(&self) -> String;
    fn to_attr_value_string(&self) -> String;
    fn b(&self) -> BuilderVariant;
}

impl BuilderShared for BuilderVariant {
    fn to_attr_key_string(&self) -> String {
        match self {
            BuilderVariant::On(b) => b.to_attr_key_string(),
            BuilderVariant::Intersects(b) => b.to_attr_key_string(),
        }
    }

    fn to_attr_value_string(&self) -> String {
        match self {
            BuilderVariant::On(b) => b.to_attr_value_string(),
            BuilderVariant::Intersects(b) => b.to_attr_value_string(),
        }
    }

    fn b(&self) -> BuilderVariant {
        self.clone()
    }
}

impl Builder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn e(self, event: &str) -> BuilderOn {
        BuilderOn::new(event)
    }

    pub fn input(self) -> BuilderOn {
        self.e("input")
    }

    pub fn click(self) -> BuilderOn {
        self.e("click")
    }

    pub fn load(self) -> BuilderOn {
        self.e("load")
    }

    pub fn store_changed(self) -> BuilderOn {
        self.e("store-changed")
    }

    pub fn raf(self) -> BuilderOn {
        self.e("raf")
    }

    pub fn intersects(self) -> BuilderIntersects {
        BuilderIntersects::new()
    }
}

impl BuilderIntersects {
    pub fn new() -> Self {
        Self {
            event: "intersects".to_string(),
            modifiers: vec![],
            actions: vec![],
        }
    }

    pub fn get(mut self, url: &str) -> Self {
        self.actions.push(js_get(url));
        self
    }
}

impl BuilderShared for BuilderIntersects {
    fn to_attr_key_string(&self) -> String {
        let modifiers_str = self.modifiers.join(".");
        let attr_str = if modifiers_str.is_empty() {
            self.event.clone()
        } else {
            format!("{}.{}", self.event, modifiers_str)
        };
        format!("data-on-{}", attr_str)
    }

    fn to_attr_value_string(&self) -> String {
        self.actions.join("; ")
    }

    fn b(&self) -> BuilderVariant {
        BuilderVariant::Intersects(self.clone())
    }
}

impl BuilderOn {
    pub fn new(event: &str) -> Self {
        Self {
            event: event.to_string(),
            modifiers: vec![],
            actions: vec![],
        }
    }
    pub fn debounce(mut self, duration: Duration) -> Self {
        self.modifiers
            .push(format!("debounce_{}ms", duration.as_millis()));
        self
    }

    pub fn once(mut self) -> Self {
        self.modifiers.push("once".to_string());
        self
    }

    pub fn passive(mut self) -> Self {
        self.modifiers.push("passive".to_string());
        self
    }

    pub fn capture(mut self) -> Self {
        self.modifiers.push("capture".to_string());
        self
    }

    pub fn throttle(mut self, duration: Duration) -> Self {
        self.modifiers
            .push(format!("throttle_{}ms", duration.as_millis()));
        self
    }

    pub fn window(mut self) -> Self {
        self.modifiers.push("window".to_string());
        self
    }

    pub fn get(mut self, url: &str) -> Self {
        self.actions.push(js_get(url));
        self
    }

    pub fn patch(mut self, url: &str) -> Self {
        self.actions.push(js_patch(url));
        self
    }

    pub fn post(mut self, url: &str) -> Self {
        self.actions.push(js_post(url));
        self
    }

    pub fn js(mut self, script: &str) -> Self {
        self.actions.push(script.to_string());
        self
    }

    pub fn push_url(mut self, url: &str) -> Self {
        self.actions
            .push(format!("window.history.pushState(null, '', '{}');", url));
        self
    }

    pub fn push_then_get(self, url: &str) -> Self {
        self.push_url(url).get(url)
    }
}

impl BuilderShared for BuilderOn {
    fn to_attr_key_string(&self) -> String {
        let modifiers_str = self.modifiers.join(".");
        let attr_str = if modifiers_str.is_empty() {
            self.event.clone()
        } else {
            format!("{}.{}", self.event, modifiers_str)
        };
        format!("data-on-{}", attr_str)
    }

    fn to_attr_value_string(&self) -> String {
        self.actions.join("; ")
    }

    fn b(&self) -> BuilderVariant {
        BuilderVariant::On(self.clone())
    }
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

    pub fn on(self, b: impl FnOnce(Builder) -> BuilderVariant) -> Self {
        let event = Builder::new();
        let modifiers = b(event);

        let key = modifiers.to_attr_key_string();
        let value = modifiers.to_attr_value_string();

        self.attr_unsafe(&key, &value)
    }

    pub fn data_persist(self, value: &str) -> Self {
        self.attr_unsafe("data-persist", value)
    }

    pub fn data_class(self, value: &str) -> Self {
        self.attr_unsafe("data-class", value)
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
