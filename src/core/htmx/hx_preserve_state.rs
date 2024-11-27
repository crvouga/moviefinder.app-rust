use crate::core::html::Elem;

const SCRIPT: &str = r#"
document.addEventListener("DOMContentLoaded", function () {
  htmx.defineExtension("preserve-state", {
    onEvent: onEvent,
  });
});

function onEvent(name, evt) {
  if (name === "htmx:beforeSwap") {
    htmx.preserveState = [];
    const elements = document.querySelectorAll("[data-preserve-state]");
    elements.forEach((element) => {
      const key = element.getAttribute("data-preserve-state");
      if (key) {
        htmx.preserveState.push({
          key: key,
          value: element.value || element.innerHTML,
          isFocused: element === document.activeElement,
          selectionStart: element.selectionStart || null,
          selectionEnd: element.selectionEnd || null,
        });
      }
    });
    return;
  }
  
  if (name === "htmx:afterSwap") {
    if (!htmx.preserveState) {
      return;
    }

    htmx.preserveState.forEach((preserveState) => {
      const element = document.querySelector(`[data-preserve-state="${preserveState.key}"]`); 
      if (!element) {
        return;
      }

      if ("value" in element) {
        element.value = preserveState.value;
      } else {
        element.innerHTML = preserveState.value;
      }

      if (preserveState.isFocused) {
        element.focus();
      }

      if (
        preserveState.selectionStart !== null &&
        preserveState.selectionEnd !== null &&
        "setSelectionRange" in element
      ) {
        element.setSelectionRange(
          preserveState.selectionStart,
          preserveState.selectionEnd
        );
      }
    });
    htmx.preserveState = [];
    return;
  }
}
"#;

impl Elem {
    pub fn js_htmx_preserve_state(self) -> Self {
        self.child_text_unsafe(SCRIPT)
    }

    pub fn hx_ext_preserve_state(self) -> Self {
        self.hx_ext("preserve-state")
    }

    pub fn hx_preserve_state(self, key: &str) -> Self {
        self.attr("data-preserve-state", key)
    }
}
