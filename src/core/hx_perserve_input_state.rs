use super::html::Elem;

const HX_PRESERVE_FOCUS_SCRIPT: &str = r#"
function waitForHtmx() {
  if (window.htmx) {
    defineExtension();
    return
  }
  setTimeout(waitForHtmx, 50); 
}

waitForHtmx();

function defineExtension() {
  htmx.defineExtension('preserve-input-state', {
    onEvent: onEvent
  });
}

function onEvent(name, evt) {
  switch(name) {
    case 'htmx:beforeSwap': {
      htmx.focusedElement = null;
      if (isInputElement(document.activeElement)) {
        htmx.focusedElement = {
          id: document.activeElement.id,
          value: document.activeElement.value,
          selectionStart: document.activeElement.selectionStart,
          selectionEnd: document.activeElement.selectionEnd,
        };
      }
      return;
    }
    case 'htmx:afterSwap': {
      if (!htmx.focusedElement) {
        return;
      }
      const focusedElement = htmx.focusedElement;
      htmx.focusedElement = null;
      const element = document.getElementById(focusedElement.id);
      if(!element) {
        return;
      }
      element.value = focusedElement.value;
      element.focus();
      if (focusedElement.selectionStart !== null && focusedElement.selectionEnd !== null) {
        element.setSelectionRange(
            focusedElement.selectionStart, 
            focusedElement.selectionEnd
        );
      }
      return;
    }
  }
}

function isInputElement(element) {
  return element && (element.tagName === 'INPUT' || element.tagName === 'TEXTAREA')
}
"#;

impl Elem {
    pub fn js_htmx_preserve_input_state(self) -> Self {
        self.child_unsafe_text(HX_PRESERVE_FOCUS_SCRIPT)
    }

    pub fn hx_ext_preserve_input_state(self) -> Self {
        self.hx_ext("preserve-input-state")
    }
}
