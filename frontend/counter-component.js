class CounterComponent extends HTMLElement {
    constructor() {
        super();
        this._count = 0;
        this.render();
    }

    render() {
        this.innerHTML = `
            <span>${this._count}</span>
            <button onclick="this.parentElement.increment()">+</button>
            <button onclick="this.parentElement.decrement()">-</button>
        `;
    }

    increment() {
        this._count++;
        this.render();
    }

    decrement() {
        this._count--;
        this.render();
    }
}

customElements.define('counter-component', CounterComponent);