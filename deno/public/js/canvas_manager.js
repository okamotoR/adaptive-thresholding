class CanvasManager {
    constructor(canvasEl,containerEl) {
        this.canvasEl = canvasEl;
        this.containerEl = containerEl;
        this.ctx = canvasEl.getContext('2d');
        this.ctx.save();// scale=1をセーブ

        this.tmpCanvasEl = document.createElement('canvas');
        this.tmpCtx = this.tmpCanvasEl.getContext('2d');
    }

    loadImage(imageData, width, height) {
        this.tmpCanvasEl.width = width;
        this.tmpCanvasEl.height = height;
        this.tmpCtx.putImageData(imageData,0,0);
        this.ctx.restore();// scale=1に復元
        this.ctx.clearRect(0, 0, this.ctx.canvas.clientWidth, this.ctx.canvas.clientHeight);
        this.ctx.save();// scale=1をセーブ

        const lessScaleRatio = Math.min(
            this.containerEl.clientWidth / width,
            this.containerEl.clientHeight / height
        );
        console.log(lessScaleRatio)
        this.canvasEl.width = width * lessScaleRatio;
        this.canvasEl.height = height * lessScaleRatio;
        this.ctx.scale(lessScaleRatio,lessScaleRatio);
        this.ctx.drawImage(this.tmpCanvasEl, 0, 0);
    }
}

export default CanvasManager;