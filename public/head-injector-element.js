// @ts-check

class HeadInjector extends HTMLElement {
  connectedCallback() {
    const headElements = Array.from(this.children);
    /**
     * @type {HTMLScriptElement[]}
     */
    const scriptsToLoad = headElements.flatMap((node) =>
      node instanceof HTMLScriptElement &&
      !this.isDuplicateScript(node) &&
      !this.isScriptLoaded(node)
        ? [node]
        : []
    );

    if (scriptsToLoad.length === 0) {
      // Emit the event immediately if all scripts are already loaded
      this.dispatchEvent(
        new CustomEvent("head-injector-loaded", { bubbles: true })
      );
      return;
    }

    const scriptPromises = scriptsToLoad.map((script) =>
      this.loadScript(script)
    );

    Promise.all(scriptPromises).then(() => {
      headElements.forEach((element) => {
        if (!(element instanceof Element)) {
          return;
        }

        if (element.tagName === "SCRIPT") {
          return;
        }

        if (!this.isDuplicateElement(element)) {
          document.head.appendChild(element);
        }
      });

      this.dispatchEvent(
        new CustomEvent("head-injector-loaded", { bubbles: true })
      );
    });
  }

  /**
   * @param {HTMLScriptElement} script
   * @returns {Promise<void>}
   */
  loadScript(script) {
    return new Promise((resolve, reject) => {
      const newScript = document.createElement("script");

      Array.from(script.attributes).forEach((attr) => {
        newScript.setAttribute(attr.name, attr.value);
      });

      if (script.textContent) {
        newScript.textContent = script.textContent;
      }

      const isAlreadyLoaded =
        script.src && document.querySelector(`script[src="${script.src}"]`);

      if (isAlreadyLoaded) {
        return resolve();
      }

      newScript.onload = () => resolve();
      newScript.onerror = () =>
        reject(new Error(`Failed to load script: ${newScript.src}`));

      document.head.appendChild(newScript);
    });
  }

  /**
   * @param {HTMLScriptElement} script
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
   * @param {HTMLScriptElement} script
   * @returns {boolean}
   */
  isScriptLoaded(script) {
    if (script.src) {
      return !!document.querySelector(`script[src="${script.src}"]`);
    }
    return false; // Inline scripts are not considered already loaded
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
