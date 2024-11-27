// @ts-check

class HeadInjector extends HTMLElement {
  connectedCallback() {
    while (this.firstChild) {
      const child = this.firstChild.cloneNode(true);
      this.removeChild(this.firstChild);

      if (!(child instanceof Element)) {
        continue;
      }

      if (child.tagName === "SCRIPT" && !this.isDuplicateScript(child)) {
        this.executeScript(child);
        continue;
      }

      if (!this.isDuplicateElement(child)) {
        document.head.appendChild(child);
        continue;
      }
    }
  }

  /**
   * @param {Element} script
   * @returns {void}
   */
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

  /**
   * @param {Element} script
   * @returns {boolean}
   */
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
