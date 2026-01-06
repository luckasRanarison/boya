import { create } from "zustand";
import { instance } from "../lib/gba";

type DebuggerStore = {
  cycles: bigint;
  lastCycle?: number;
  breakpoints: number[];
  romLoaded: boolean;
  running: boolean;
  canvas?: { context: CanvasRenderingContext2D; imageData: ImageData };
  fps: number;

  setCanvas: (params: {
    context: CanvasRenderingContext2D;
    imageData: ImageData;
  }) => void;

  run: () => void;
  pause: () => void;
  stepInto: () => void;
  setBreakpoints: (breakPoints: number[]) => void;
  loadRom: (rom: Uint8Array) => void;
};

export const useDebuggerStore = create<DebuggerStore>((set, get) => ({
  cycles: BigInt(0),
  running: false,
  paused: false,
  romLoaded: false,
  breakpoints: [],
  fps: 0,

  setCanvas: (canvas) => set((prev) => ({ ...prev, canvas })),
  setBreakpoints: (breakpoints) => set((prev) => ({ ...prev, breakpoints })),
  pause: () => set((prev) => ({ ...prev, running: false, paused: true })),

  loadRom: (rom) => {
    instance.loadRom(rom);
    instance.boot();
    set((prev) => ({ ...prev, cycles: instance.cycles(), romLoaded: true }));
  },

  run: () => {
    let frameCount = 0;
    let lastTime = 0;
    let lastFpsUpdate = 0;

    if (get().running) return;

    const renderingLoop = (timestamp: number) => {
      const { running, canvas, breakpoints } = get();

      if (!running) return;

      if (!lastTime) {
        lastTime = timestamp;
      }

      const delta = timestamp - lastFpsUpdate;

      if (delta >= 1000) {
        const fps = Math.ceil((frameCount * 1000) / delta);

        lastFpsUpdate = timestamp;
        frameCount = 0;
        set((prev) => ({ ...prev, fps }));
      }

      lastTime = timestamp;
      frameCount++;

      if (breakpoints.length) {
        const cycles = instance.stepFrameWithBreakpoints(
          new Uint32Array(breakpoints),
        );

        set((prev) => ({ ...prev, lastCycle: cycles }));

        if (breakpoints.includes(instance.pc()) || !get().running) {
          return set((prev) => ({ ...prev, running: false }));
        }
      } else {
        instance.stepFrame();
        instance.updateFrameBuffer();
      }

      if (canvas) {
        const pixels = canvas.imageData.data;
        instance.writeFrameBuffer(pixels as unknown as Uint8Array);
        canvas.context.putImageData(canvas.imageData, 0, 0);
      }

      set((prev) => ({
        ...prev,
        cycles: instance.cycles(),
        lastCycle: undefined,
      }));
      requestAnimationFrame(renderingLoop);
    };

    set((prev) => ({ ...prev, running: true }));
    renderingLoop(lastTime);
  },

  stepInto: () => {
    const count = instance.debugSyncedStep();

    set((prev) => ({
      ...prev,
      lastCycle: count,
      cycles: prev.cycles + BigInt(count),
    }));
  },
}));
