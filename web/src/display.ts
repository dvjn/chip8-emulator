export class Display {
  ctx: CanvasRenderingContext2D;

  constructor() {
    const canvas = document.querySelector("#display") as HTMLCanvasElement;
    canvas.width = 64;
    canvas.height = 32;

    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("cannot get canvas context");

    this.ctx = ctx;
  }

  render(buffer: Array<boolean>) {
    const imageData = this.ctx.createImageData(64, 32);

    for (let i = 0; i < buffer.length; i++) {
      imageData.data[i * 4] = 0xff;
      imageData.data[i * 4 + 1] = 0xff;
      imageData.data[i * 4 + 2] = 0xff;
      imageData.data[i * 4 + 3] = buffer[i] ? 0xff : 0x00;
    }

    this.ctx.putImageData(imageData, 0, 0);
  }
}
