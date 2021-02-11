class SliderInputManager {
    constructor(inputEl, rangeEl) {
        this.inputEl = inputEl;
        this.rangeEl = rangeEl;
    }

    changeValue(value) {
        this.inputEl.value = value;
        this.rangeEl.value = value;
    }
}

export default SliderInputManager;