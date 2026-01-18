import { create } from "zustand";
import { instance } from "@/lib/gba";
import { FrameCounter } from "@/utils";
import type { Keymap } from "@/lib/keymap";

type DebuggerStore = {
  cycles: bigint;
  lastCycle?: number;
  breakpoints: number[];
  romLoaded: boolean;
  running: boolean;
  keypad: number;
  canvas?: { context: CanvasRenderingContext2D; imageData: ImageData };
  fps: number;
  paused: boolean;

  run: () => void;
  pause: () => void;
  stepInto: () => void;
  setCanvas: (canvas: HTMLCanvasElement) => void;
  setBreakpoints: (breakPoints: number[]) => void;
  loadRom: (rom: Uint8Array) => void;
  createKeyHandler: (keymap: Keymap) => (event: KeyboardEvent) => void;
};

export const useDebuggerStore = create<DebuggerStore>((set, get) => ({
  cycles: BigInt(0),
  running: false,
  paused: false,
  romLoaded: false,
  keypad: 0x3ff,
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
    const startTime = Date.now();

    const intervalId = setInterval(() => {
      const { running, canvas, breakpoints, paused } = get();

      if (!running || paused) {
        return clearInterval(intervalId);
      }

      const ellapsed = Date.now() - startTime;

      frameCounter.onFrame(ellapsed, {
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
    }, 1000 / 60);

    set((prev) => ({ ...prev, running: true, paused: false }));
  },

  stepInto: () => {
    const count = instance.debugSyncedStep();

    set((prev) => ({
      ...prev,
      lastCycle: count,
      cycles: prev.cycles + BigInt(count),
    }));
  },

  createKeyHandler: (keymap) => (event) => {
    const key = keymap[event.code];

    if (!key) return;

    switch (event.type) {
      case "keyup":
        set((prev) => ({ ...prev, keypad: prev.keypad | key }));
        break;
      case "keydown":
        set((prev) => ({ ...prev, keypad: prev.keypad & ~key }));
        break;
      default:
    }

    instance.setKeyinput(get().keypad);
  },
}));
