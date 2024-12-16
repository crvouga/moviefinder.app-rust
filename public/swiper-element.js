class SwiperContainer extends HTMLElement {
  constructor() {
    super();
    this.currentSlide = 0;
    this.startY = 0;
    this.endY = 0;
    this.isSwiping = false;
    this.attachShadow({ mode: "open" });
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          position: relative;
          overflow: hidden;
          width: 100%;
          height: 100%;
        }
        .container {
          display: flex;
          flex-direction: column;
          height: 100%;
          width: 100%;
          transition: transform 0.3s ease-out;
          will-change: transform;
        }
        ::slotted(swiper-slide) {
          flex-shrink: 0;
          height: 100%;
          width: 100%;
        }
      </style>
      <div class="container">
        <slot></slot>
      </div>
    `;
    this.container = this.shadowRoot.querySelector(".container");
  }

  connectedCallback() {
    this.addEventListener("touchstart", this.onTouchStart.bind(this));
    this.addEventListener("touchmove", this.onTouchMove.bind(this));
    this.addEventListener("touchend", this.onTouchEnd.bind(this));
  }

  disconnectedCallback() {
    this.removeEventListener("touchstart", this.onTouchStart.bind(this));
    this.removeEventListener("touchmove", this.onTouchMove.bind(this));
    this.removeEventListener("touchend", this.onTouchEnd.bind(this));
  }

  onTouchStart(event) {
    if (this.isSwiping) return;
    this.startY = event.touches[0].clientY;
  }

  onTouchMove(event) {
    if (this.isSwiping) return;
    this.endY = event.touches[0].clientY;
  }

  onTouchEnd() {
    if (this.isSwiping) return;
    const deltaY = this.startY - this.endY;
    const threshold = 50; // Minimum swipe distance to trigger a slide

    if (deltaY > threshold && this.currentSlide < this.slides.length - 1) {
      this.currentSlide++;
    } else if (deltaY < -threshold && this.currentSlide > 0) {
      this.currentSlide--;
    }

    this.updateSlidePosition();
  }

  updateSlidePosition() {
    this.isSwiping = true;
    const translateY = -this.currentSlide * 100;
    this.container.style.transform = `translateY(${translateY}%)`;
    setTimeout(() => {
      this.isSwiping = false;
    }, 300); // Match the transition duration
  }

  get slides() {
    return this.querySelectorAll("swiper-slide");
  }
}

class SwiperSlide extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          height: 100%;
          width: 100%;
          overflow: hidden;
        }
      </style>
      <slot></slot>
    `;
  }
}

customElements.define("swiper-container", SwiperContainer);
customElements.define("swiper-slide", SwiperSlide);
