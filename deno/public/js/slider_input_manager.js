class SliderInputManager {
    constructor(inputEl, rangeEl, value) {
        this.inputEl = inputEl;
        this.rangeEl = rangeEl;
        this.changeValue(value)
    }

    changeValue(value) {
        this.inputEl.value = value;
        this.rangeEl.value = value;
        this.value = value;
    }
}

export default SliderInputManager;