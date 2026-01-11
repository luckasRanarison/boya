import { create } from "zustand";
import { instance } from "@/lib/gba";
import { FrameCounter } from "@/utils";

type DebuggerStore = {
  cycles: bigint;
  lastCycle?: number;
  breakpoints: number[];
  romLoaded: boolean;
  running: boolean;
  canvas?: { context: CanvasRenderingContext2D; imageData: ImageData };
  fps: number;
  paused: boolean;

  run: () => void;
  pause: () => void;
  stepInto: () => void;
  setCanvas: (canvas: HTMLCanvasElement) => void;
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

  setBreakpoints: (breakpoints) => {
    set((prev) => ({ ...prev, breakpoints }));
  },

  setCanvas: (canvas: HTMLCanvasElement) => {
    const context = canvas.getContext("2d")!;
    const imageData = context.createImageData(240, 160);

    set((prev) => ({ ...prev, canvas: { context, imageData } }));
  },

  loadRom: (rom) => {
    instance.loadRom(rom);
    instance.boot();
    set((prev) => ({ ...prev, cycles: instance.cycles(), romLoaded: true }));
  },

  pause: () => {
    set((prev) => ({ ...prev, running: false, paused: true }));
  },

  run: () => {
    if (get().running) return;

    const frameCounter = new FrameCounter();

    const renderingLoop = (timestamp: number) => {
      const { running, canvas, breakpoints } = get();

      if (!running) return;

      frameCounter.onFrame(timestamp, {
        interval: 1000,
        callback: (fps) => set((prev) => ({ ...prev, fps })),
      });

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

        set((prev) => ({
          ...prev,
          cycles: instance.cycles(),
          lastCycle: undefined,
        }));
      }

      if (canvas) {
        const pixels = canvas.imageData.data;
        instance.writeFrameBuffer(pixels as unknown as Uint8Array);
        canvas.context.putImageData(canvas.imageData, 0, 0);
      }

      requestAnimationFrame(renderingLoop);
    };

    set((prev) => ({ ...prev, running: true }));
    renderingLoop(0);
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
