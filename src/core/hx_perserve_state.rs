use super::html::Elem;

const HX_PRESERVE_FOCUS_SCRIPT: &str = r#"
document.addEventListener("DOMContentLoaded", function () {
  htmx.defineExtension("preserve-state", {
    onEvent: onEvent,
  });
});

function onEvent(name, evt) {
  if(name === "htmx:beforeSwap") {
    htmx.preserveState = [];
    const elements = document.querySelectorAll("[data-preserve-state]");
    elements.forEach((element) => {
      htmx.preserveState.push({
        id: element.id,
        value: element.value || element.innerHTML,
        selectionStart: element.selectionStart || null,
        selectionEnd: element.selectionEnd || null,
      });
    });
    return;
  }
  if(name === "htmx:afterSwap") {
    if (!htmx.preserveState) {
      return;
    }
    htmx.preserveState.forEach((preserveState) => {
      const element = document.getElementById(preserveState.id);
      if (!element) {
        return;
      }
      if ("value" in element) {
        element.value = preserveState.value;
      } else {
        element.innerHTML = preserveState.value;
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
        self.child_unsafe_text(HX_PRESERVE_FOCUS_SCRIPT)
    }

    pub fn hx_ext_preserve_state(self) -> Self {
        self.hx_ext("preserve-state")
    }

    pub fn hx_perserve_state(self) -> Self {
        self.attr("data-preserve-state", "")
    }
}
