class Drawer extends HTMLElement {
  constructor() {
    super();
    this.backdrop = null;
    this.content = null;
    this.root = null;
    this.onKeyDown = this.onKeyDown.bind(this);
    this.attachShadow({ mode: "open" });
  }

  disconnectedCallback() {
    window.removeEventListener("keydown", this.onKeyDown);
  }

  connectedCallback() {
    window.addEventListener("keydown", this.onKeyDown);

    if (!this.shadowRoot) return;

    this.shadowRoot.innerHTML = `
    <style>
    .root {
        position: absolute;
        inset: 0;
        width: 100%;
        max-width: 100%;
        height: 100%;
        max-height: 100%;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        z-index: 100;
        background-color: rgba(0, 0, 0, 0);
        transition: background-color 0.2s ease-in-out;
    }
    .root-top {
        flex-direction: column-reverse;
    }
    .root-bottom {
        flex-direction: column;
    }
    .content {
        width: 100%;
        max-width: 100%;
        max-height: 100%;
        overflow: hidden;
        transition: transform 0.2s ease-in-out;
        transform: translateY(100%); /* Initial off-screen state */
    }
    .content-top {
        transform: translateY(-100%);
    }
    .backdrop {
        flex: 1;
        width: 100%;
        opacity: 0;
        transition: opacity 0.2s ease-in-out;
    }
    .darken-background {
        background-color: rgba(0, 0, 0, 0.75);
    }
    .visible {
        opacity: 1;
    }
    .open {
        transform: translateY(0%); /* Final visible state */
    }
    </style>
    <div class="root">
        <div class="backdrop"></div>
        <div class="content">
            <slot></slot>
        </div>
    </div>
    `;

    this.root = this.shadowRoot.querySelector(".root");
    this.backdrop = this.shadowRoot.querySelector(".backdrop");
    this.content = this.shadowRoot.querySelector(".content");

    this.backdrop?.addEventListener("click", () => {
      this.emitCloseEvent();
    });

    this.setupPosition();
    this.close();
  }

  static get observedAttributes() {
    return ["position", "open"];
  }

  setupPosition() {
    const position = this.position();
    if (this.content) {
      this.content.classList.remove("content-top", "content-bottom");
      switch (position) {
        case "bottom":
          this.content.classList.add("content-bottom");
          break;
        case "top":
          this.content.classList.add("content-top");
          break;
        default:
          break;
      }
    }
  }

  setupOpen() {
    if (this.isOpen()) {
      this.open();
    } else {
      this.close();
    }
  }

  attributeChangedCallback(name, _oldValue, _newValue) {
    if (name === "position") {
      this.setupPosition();
    }

    if (name === "open") {
      setTimeout(() => {
        this.setupOpen();
      }, 100);
    }
  }

  onKeyDown(keyboardEvent) {
    if (keyboardEvent.key === "Escape") {
      this.emitCloseEvent();
    }
  }

  emitCloseEvent() {
    this.dispatchEvent(new CustomEvent("close", { bubbles: true }));
  }

  isOpen() {
    return this.hasAttribute("open") && this.getAttribute("open") === "true";
  }

  position() {
    return this.getAttribute("position") ?? "";
  }

  open() {
    if (this.content) {
      this.content.classList.add("open");
    }
    if (this.root) {
      this.root.classList.add("darken-background");
      this.root.style.pointerEvents = "auto";
    }

    if (this.backdrop) {
      this.backdrop.classList.add("visible");
    }
  }

  close() {
    if (this.content) {
      this.content.classList.remove("open");
    }
    if (this.root) {
      this.root.classList.remove("darken-background");
      this.root.style.pointerEvents = "none";
    }
    if (this.backdrop) {
      this.backdrop.classList.remove("visible");
    }
  }
}

customElements.define("drawer-element", Drawer);
