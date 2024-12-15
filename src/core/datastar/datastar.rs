use std::{collections::HashMap, time::Duration};

use crate::core::{
    dynamic_data::{DynamicData, DynamicDataHashMap},
    html::{code, pre, Elem},
    http::{
        json_data::JsonData,
        request::Request,
        response_writer::ResponseWriter,
        server_sent_event::{sse, ServerSentEvent},
    },
    js::Js,
    url_encoded,
};

fn fallback_empty_string(value: &str) -> String {
    if value.trim().is_empty() {
        "''".to_string()
    } else {
        value.to_string()
    }
}

impl Js {
    pub fn sse(url: &str) -> String {
        "sse('URL', { method: 'post' })".replace("URL", url)
    }

    pub fn dot_value(value: &str) -> String {
        format!("{}.value", value)
    }
}

pub trait Attr {
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

    pub fn class(mut self, class: &str, signal: &str) -> Self {
        self.classes.insert(class.to_string(), signal.to_string());
        self
    }

    pub fn maybe_class(mut self, class: &str, signal: &Option<String>) -> Self {
        match signal {
            Some(signal) => self.classes.insert(class.to_string(), signal.to_string()),
            None => None,
        };
        self
    }
}

impl Attr for DataClass {
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

    pub fn sse(mut self, url: &str) -> Self {
        self.actions.push(Js::sse(url));
        self
    }
}

impl Attr for DataIntersects {
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

    // pub fn keydown(self) -> Self {
    //     Self {
    //         event: "keydown".to_string(),
    //         modifiers: vec![],
    //         js: vec![],
    //     }
    // }

    pub fn submit(self) -> Self {
        Self {
            event: "submit".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn change(self) -> Self {
        Self {
            event: "change".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn prevent_default(self) -> Self {
        self.js("evt.preventDefault()")
    }

    pub fn load(self) -> Self {
        Self {
            event: "load".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    pub fn signals_changed(self) -> Self {
        Self {
            event: "signals-change".to_string(),
            modifiers: vec![],
            js: vec![],
        }
    }

    // pub fn raf(self) -> Self {
    //     Self {
    //         event: "raf".to_string(),
    //         modifiers: vec![],
    //         js: vec![],
    //     }
    // }

    pub fn debounce(mut self, duration: Duration) -> Self {
        self.modifiers
            .push(format!("debounce.{}ms", duration.as_millis()));
        self
    }

    // pub fn once(mut self) -> Self {
    //     self.modifiers.push("once".to_string());
    //     self
    // }

    // pub fn passive(mut self) -> Self {
    //     self.modifiers.push("passive".to_string());
    //     self
    // }

    // pub fn capture(mut self) -> Self {
    //     self.modifiers.push("capture".to_string());
    //     self
    // }

    // pub fn throttle(mut self, duration: Duration) -> Self {
    //     self.modifiers
    //         .push(format!("throttle_{}ms", duration.as_millis()));
    //     self
    // }

    // pub fn window(mut self) -> Self {
    //     self.modifiers.push("window".to_string());
    //     self
    // }

    pub fn js(mut self, js: &str) -> Self {
        self.js.push(js.to_string());
        self
    }

    pub fn sse(self, url: &str) -> Self {
        self.js(&Js::sse(url))
    }

    pub fn push_then_sse(self, url: &str) -> Self {
        self.js(&Js::push_url(url)).sse(url)
    }
}

impl Attr for DataOn {
    fn attr(&self) -> (String, String) {
        let modifiers_str = self.modifiers.join(".");
        let binding = if modifiers_str.is_empty() {
            self.event.clone()
        } else {
            format!("{}__{}", self.event, modifiers_str)
        };
        let attr_str = binding.trim();

        if attr_str.is_empty() {
            return ("".to_string(), "".to_string());
        }

        let key = format!("data-on-{}", attr_str);
        let value = self.js.join("; ");
        (key, value)
    }
}

impl Elem {
    pub fn src_datastar(self) -> Self {
        self.src("https://cdn.jsdelivr.net/gh/starfederation/datastar/bundles/datastar.js")
    }

    pub fn data_bind(self, value: &str) -> Self {
        self.attr_unsafe(&format!("data-bind-{}", value), "")
    }

    // pub fn data_ref(self, value: &str) -> Self {
    //     self.attr_unsafe("data-ref", value)
    // }

    pub fn data_signals(self, value: &str) -> Self {
        self.attr_unsafe("data-signals", value)
    }

    pub fn data_signal(self, key: &str, value: &str) -> Self {
        let value_final = &fallback_empty_string(value);
        self.attr_unsafe(&format!("data-signals-{}", key), value_final)
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

    // pub fn data_persist(self, value: &str) -> Self {
    //     self.attr_unsafe("data-persist", value)
    // }

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

    pub fn data_attributes(self, attr: &str, value: &str) -> Self {
        self.attr_unsafe(&format!("data-attributes-{}", attr), value)
    }

    pub fn data_show(self, value: &str) -> Self {
        self.attr_unsafe("data-show", value)
    }

    // pub fn data_computed(self, name: &str, value: &str) -> Self {
    //     self.attr_unsafe(&format!("data-computed-{}", name), value)
    // }

    pub fn child_signals_json(self, debug: bool) -> Self {
        if debug {
            self.child(code().child(pre().data_text("ctx.signals.JSON()")))
        } else {
            self
        }
    }
}

impl ServerSentEvent {
    // pub fn fragment(self, elem: Elem) -> MergeFragment {
    //     MergeFragment::new(self, elem)
    // }

    pub fn event_merge_fragments(&mut self) -> &mut Self {
        self.event("datastar-merge-fragments")
    }

    pub fn event_merge_signals(&mut self) -> &mut Self {
        self.event("datastar-merge-signals")
    }

    pub fn data_signals(&mut self, value: &str) -> &mut Self {
        self.data(&format!("signals {}", value))
    }

    // pub fn data_only_if_missing(&mut self, value: bool) -> &mut Self {
    //     if value {
    //         self.data("onlyIfMissing true")
    //     } else {
    //         self.data("onlyIfMissing false")
    //     }
    // }

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

        fn clean_string(input: &str) -> String {
            input
                .replace('\t', " ")
                .replace('\n', " ")
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join(" ")
        }

        let data = format!("fragments {}", clean_string(&rendered));

        self.data(&data)
    }

    pub fn event_execute_script(&mut self) -> &mut Self {
        self.event("datastar-execute-script")
    }

    pub fn data_script(&mut self, script: &str) -> &mut Self {
        let data = format!("script {}", script);
        self.data(&data)
    }
}

pub struct Fragments {
    sse: ServerSentEvent,
}

pub fn fragments(elem: Elem) -> Fragments {
    Fragments::new(elem)
}

impl Fragments {
    fn new(elem: Elem) -> Self {
        Self {
            sse: sse().event_merge_fragments().data_fragments(elem).clone(),
        }
    }

    pub fn merge_mode(&mut self, mode: &str) -> &mut Self {
        self.sse.data_merge_mode(mode);
        self
    }

    pub fn merge_mode_outer(&mut self) -> &mut Self {
        self.merge_mode("outer")
    }

    pub fn merge_mode_before(&mut self) -> &mut Self {
        self.merge_mode("before")
    }

    pub fn selector(&mut self, selector: &str) -> &mut Self {
        self.sse.data_selector(selector);
        self
    }

    pub async fn send(&mut self, w: &mut ResponseWriter) -> Result<(), std::io::Error> {
        self.sse.send(w).await?;
        Ok(())
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

    pub async fn send_signals(&mut self, signals: Vec<(&str, &str)>) -> Result<(), std::io::Error> {
        let key_value_pairs = signals
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join(", ");

        let js_obj = format!("{{{}}}", key_value_pairs);

        sse()
            .event_merge_signals()
            .data_signals(&js_obj)
            .send(self)
            .await
    }

    pub async fn send_signal(&mut self, key: &str, value: &str) -> Result<(), std::io::Error> {
        self.send_signals(vec![(key, value)]).await
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

    pub fn datastar_params(self: &Self) -> DynamicDataHashMap {
        let datastar_params = self.url.query_params.get_first("datastar");

        if let Some(urlencoded_json) = datastar_params {
            let json = url_encoded::decode(&urlencoded_json);
            JsonData::from_string(&json).params
        } else {
            self.params()
        }
    }
}
