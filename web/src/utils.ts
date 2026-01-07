export function formatHex(value: number, width = 8) {
  return `0x${value.toString(16).padStart(width, "0")}`;
}

export class FrameCounter {
  lastTime: number;
  lastFpsUpdate: number;
  frameCount: number;

  constructor() {
    this.lastTime = 0;
    this.lastFpsUpdate = 0;
    this.frameCount = 0;
  }

  onFrame(
    timestamp: number,
    {
      interval,
      callback,
    }: { interval: number; callback: (fps: number) => void },
  ) {
    if (this.lastTime === 0) {
      this.lastTime = timestamp;
    }

    const delta = timestamp - this.lastFpsUpdate;

    if (delta >= interval) {
      const fps = Math.ceil((this.frameCount * interval) / delta);

      this.lastFpsUpdate = timestamp;
      this.frameCount = 0;

      callback(fps);
    }

    this.lastTime = timestamp;
    this.frameCount += 1;
  }
}
