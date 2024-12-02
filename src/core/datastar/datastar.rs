use std::{collections::HashMap, time::Duration};

use crate::core::{
    html::{code, pre, Elem},
    http::{
        json_data::JsonData,
        request::Request,
        response_writer::ResponseWriter,
        server_sent_event::{sse, ServerSentEvent},
    },
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

pub fn signal(value: &str) -> String {
    format!("(${})", value)
}

pub fn js_not(value: &str) -> String {
    format!("!({})", value)
}

pub fn js_replace_url(url: &str) -> String {
    format!("window.history.replaceState(null, '', '{}');", url)
}

pub fn js_push_url(url: &str) -> String {
    format!("window.history.pushState(null, '', '{}');", url)
}

pub trait Builder {
    fn attr(&self) -> (String, String);
}

#[derive(Debug, Clone, Default)]
pub struct DataClass {
    classes: HashMap<String, String>,
}

impl DataClass {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
        }
    }

    pub fn c(mut self, class: &str, signal: &str) -> Self {
        self.classes.insert(class.to_string(), signal.to_string());
        self
    }
}

impl Builder for DataClass {
    fn attr(&self) -> (String, String) {
        let mut classes = vec![];
        for (class, signal) in &self.classes {
            classes.push(format!("'{}': {}", class, signal));
        }
        let classes_str = format!("{{{}}}", classes.join(","));
        ("data-class".to_string(), classes_str)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DataIntersects {
    modifiers: Vec<String>,
    actions: Vec<String>,
}

impl DataIntersects {
    pub fn new() -> Self {
        Self {
            modifiers: vec![],
            actions: vec![],
        }
    }

    pub fn get(mut self, url: &str) -> Self {
        self.actions.push(js_get(url));
        self
    }
}

impl Builder for DataIntersects {
    fn attr(&self) -> (String, String) {
        let value = self.actions.join("; ");
        let modifiers_str = self.modifiers.join(".");
        let event = "intersects".to_string();
        let attr_str = if modifiers_str.is_empty() {
            event
        } else {
            format!("{}.{}", event, modifiers_str)
        };
        let key = format!("data-{}", attr_str);
        (key, value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DataOn {
    event: String,
    modifiers: Vec<String>,
    js: Vec<String>,
}

impl DataOn {
    pub fn new(event: &str) -> Self {
        Self {
            event: event.to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn e(mut self, event: &str) -> Self {
        self.event = event.to_string();
        self
    }

    pub fn input(self) -> Self {
        Self {
            event: "input".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn click(self) -> Self {
        Self {
            event: "click".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn keydown(self) -> Self {
        Self {
            event: "keydown".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn submit(self) -> Self {
        Self {
            event: "submit".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn prevent_default(mut self) -> Self {
        self.js("evt.preventDefault()")
    }

    pub fn load(self) -> Self {
        Self {
            event: "load".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn store_changed(self) -> Self {
        Self {
            event: "store-changed".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn raf(self) -> Self {
        Self {
            event: "raf".to_string(),
            modifiers: vec![],
            js: vec![],
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
        self.js.push(js_get(url));
        self
    }

    pub fn patch(mut self, url: &str) -> Self {
        self.js.push(js_patch(url));
        self
    }

    pub fn post(mut self, url: &str) -> Self {
        self.js.push(js_post(url));
        self
    }

    pub fn js(mut self, script: &str) -> Self {
        self.js.push(script.to_string());
        self
    }

    pub fn log(mut self, message: &str) -> Self {
        self.js.push(format!("console.log('{}')", message));
        self
    }

    pub fn push_url(mut self, url: &str) -> Self {
        self.js
            .push(format!("window.history.pushState(null, '', '{}');", url));
        self
    }

    pub fn push_then_get(self, url: &str) -> Self {
        self.push_url(url).get(url)
    }
}

impl Builder for DataOn {
    fn attr(&self) -> (String, String) {
        let modifiers_str = self.modifiers.join(".");
        let attr_str = if modifiers_str.is_empty() {
            self.event.clone()
        } else {
            format!("{}.{}", self.event, modifiers_str)
        };
        let key = format!("data-on-{}", attr_str);
        let value = self.js.join("; ");
        (key, value)
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

    pub fn data_on(self, b: impl FnOnce(DataOn) -> DataOn) -> Self {
        let builder = b(DataOn::new(""));
        let (key, value) = builder.attr();
        self.attr_unsafe(&key, &value)
    }

    pub fn data_intersects(self, b: impl FnOnce(DataIntersects) -> DataIntersects) -> Self {
        let builder = b(DataIntersects::new());
        let (key, value) = builder.attr();
        self.attr_unsafe(&key, &value)
    }

    pub fn data_persist(self, value: &str) -> Self {
        self.attr_unsafe("data-persist", value)
    }

    pub fn data_class(self, b: impl FnOnce(DataClass) -> DataClass) -> Self {
        let builder = b(DataClass::new());
        let (key, value) = builder.attr();
        self.attr_unsafe(&key, &value)
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

    pub fn debug_store(self, debug: bool) -> Self {
        if debug {
            self.child(code().child(pre().data_text("JSON.stringify(ctx.store(), null, 2)")))
        } else {
            self
        }
    }
}

struct MergeFragment {
    sse: ServerSentEvent,
    fragment: Elem,
}

impl MergeFragment {
    pub fn new(sse: ServerSentEvent, fragment: Elem) -> Self {
        Self { fragment, sse }
    }

    pub fn fragment(&mut self, elem: Elem) -> &mut Self {
        self.sse.data_fragments(elem);
        self
    }

    pub fn merge_mode_outer(&mut self) -> &mut Self {
        self.sse.data_merge_mode_outer();
        self
    }

    pub fn merge_mode_before(&mut self) -> &mut Self {
        self.sse.data_merge_mode_before();
        self
    }

    pub fn selector(&mut self, selector: &str) -> &mut Self {
        self.sse.data_selector(selector);
        self
    }

    pub fn selector_id(&mut self, id: &str) -> &mut Self {
        self.sse.data_selector_id(id);
        self
    }

    pub fn only_if_missing(&mut self, value: bool) -> &mut Self {
        self.sse.data_only_if_missing(value);
        self
    }
}

impl ServerSentEvent {
    pub fn fragment(self, elem: Elem) -> MergeFragment {
        MergeFragment::new(self, elem)
    }

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
        self.data_script(&js_replace_url(url))
    }
}

impl ResponseWriter {
    pub async fn send_fragment(&mut self, elem: Elem) -> Result<(), std::io::Error> {
        sse()
            .event_merge_fragments()
            .data_fragments(elem)
            .send(self)
            .await
    }

    pub async fn send_signals(&mut self, value: &str) -> Result<(), std::io::Error> {
        sse()
            .event_merge_signals()
            .data_signals(value)
            .send(self)
            .await
    }

    pub async fn send_replace_url(&mut self, url: &str) -> Result<(), std::io::Error> {
        self.send_script(&js_replace_url(url)).await
    }

    pub async fn send_push_url(&mut self, url: &str) -> Result<(), std::io::Error> {
        self.send_script(&js_push_url(url)).await
    }

    pub async fn send_focus(&mut self, selector: &str) -> Result<(), std::io::Error> {
        sse()
            .event_execute_script()
            .data_script(&format!("document.querySelector('{}').focus()", selector))
            .send(self)
            .await
    }

    pub async fn send_script(&mut self, script: &str) -> Result<(), std::io::Error> {
        sse()
            .event_execute_script()
            .data_script(script)
            .send(self)
            .await?;

        Ok(())
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
