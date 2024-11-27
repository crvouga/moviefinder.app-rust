// @ts-check

class HeadInjector extends HTMLElement {
  connectedCallback() {
    while (this.firstChild) {
      const child = this.firstChild.cloneNode(true);
      if (child instanceof Element && child.tagName === "SCRIPT") {
        if (!this.isDuplicateScript(child)) {
          this.executeScript(child);
        }
      } else if (child instanceof Element && !this.isDuplicateElement(child)) {
        document.head.appendChild(child);
      }
      this.removeChild(this.firstChild);
    }
  }

  executeScript(script) {
    const newScript = document.createElement("script");

    Array.from(script.attributes).forEach((attr) => {
      newScript.setAttribute(attr.name, attr.value);
    });

    if (script.textContent) {
      newScript.textContent = script.textContent;
    }

    document.head.appendChild(newScript);
  }

  isDuplicateScript(script) {
    const existingScripts = Array.from(
      document.head.querySelectorAll("script")
    );
    return existingScripts.some((existingScript) => {
      return (
        Array.from(script.attributes).every((attr) => {
          return existingScript.getAttribute(attr.name) === attr.value;
        }) && script.textContent === existingScript.textContent
      );
    });
  }

  /**
   * @param {Element} element
   * @returns {boolean}
   */
  isDuplicateElement(element) {
    const tagName = element.tagName.toLowerCase();
    const existingElements = Array.from(
      document.head.querySelectorAll(tagName)
    );
    return existingElements.some((existingElement) => {
      return Array.from(element.attributes).every((attr) => {
        return existingElement.getAttribute(attr.name) === attr.value;
      });
    });
  }
}

customElements.define("head-injector", HeadInjector);
