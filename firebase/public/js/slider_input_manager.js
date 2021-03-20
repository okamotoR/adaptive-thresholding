class SliderInputManager {
    constructor(inputEl, rangeEl, value, max, min) {
        this.inputEl = inputEl;
        this.rangeEl = rangeEl;
        this.max = max;
        this.min = min;
        this.changeValue(value)
    }

    changeValue(value) {
        let next = value;
        if (value > this.max) {
            next = this.max;
        }
        if (value < this.min) {
            next = this.min;
        }
        this.inputEl.value = next;
        this.rangeEl.value = next;
        this.value = next;
    }

    addValue() {
        this.changeValue(this.value + 1);
    }

    minusValue() {
        this.changeValue(this.value - 1);
    }
}

export default SliderInputManager;