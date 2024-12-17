use crate::core::{html::Elem, http::server_sent_event::ServerSentEvent};

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

    pub fn data_selector(&mut self, selector: &str) -> &mut Self {
        self.data(&format!("selector {}", selector))
    }

    pub fn data_selector_id(&mut self, id: &str) -> &mut Self {
        self.data_selector(&format!("#{}", id))
    }

    pub fn data_fragments(&mut self, elem: Elem) -> &mut Self {
        let rendered = elem.render();

        let data = format!("fragments {}", clean_html(&rendered));

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

fn clean_html(input: &str) -> String {
    input
        .replace('\t', " ")
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}
