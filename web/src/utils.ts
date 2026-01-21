export function formatHex(
  value: number,
  params?: { width?: number; prefix?: string },
) {
  return `${params?.prefix ?? "0x"}${value.toString(16).padStart(params?.width ?? 8, "0")}`;
}

export function parseHex(value: string, params?: { prefix?: string }) {
  return parseInt(value.replace(params?.prefix ?? "0x", ""), 16);
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
    ellapsed: number,
    {
      interval,
      callback,
    }: { interval: number; callback: (fps: number) => void },
  ) {
    if (this.lastTime === 0) {
      this.lastTime = ellapsed;
    }

    const delta = ellapsed - this.lastFpsUpdate;

    if (delta >= interval) {
      const fps = Math.ceil((this.frameCount * interval) / delta);

      this.lastFpsUpdate = ellapsed;
      this.frameCount = 0;

      callback(fps);
    }

    this.lastTime = ellapsed;
    this.frameCount += 1;
  }
}
