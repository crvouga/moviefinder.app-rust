use crate::core::html::*;

const SCRIPT_DISABLE_SUBMIT_WHEN_UNCHANGED: &str = r#"
const form = document.currentScript.closest("form");

disableSubmitWhenUnchanged()
document.addEventListener("DOMContentLoaded", disableSubmitWhenUnchanged);

function disableSubmitWhenUnchanged() {
    const initial = new FormData(form);
    form.addEventListener("input", () => {
        const submitBtn = form.querySelector('button[type="submit"]');
        if (submitBtn) {
            submitBtn.disabled = !hasFormChanged(initial, new FormData(form)); 
        }
    });
}

function formDataToObject(formData) {
    const obj = {};
    for (const [key, value] of formData.entries()) {
        if (!obj.hasOwnProperty(key)) {
            obj[key] = [];
        }
        obj[key].push(value);
    }
    return obj;
}

function hasFormChanged(beforeFormData, afterFormData) {
    const before = formDataToObject(beforeFormData);
    const after = formDataToObject(afterFormData);
    for (const key in after) {
        const value = after[key];
        const initialValue = before[key];
        console.log({key, value, initialValue});
        
        if (JSON.stringify(value) !== JSON.stringify(initialValue)) {
            return true;
        }
    }
    return false; 
}

"#;

pub fn script_disable_submit_when_unchanged() -> Elem {
    script(&[], &[unsafe_html(SCRIPT_DISABLE_SUBMIT_WHEN_UNCHANGED)])
}
