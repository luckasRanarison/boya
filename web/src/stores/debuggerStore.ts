import { create } from "zustand";
import { instance } from "@/lib/gba";
import { FrameCounter } from "@/utils/frame";
import type { Position } from "@/utils/float";

type InstructionChache = {
  [key: number]: { value: string; size: number } | undefined;
};

type DebuggerStore = {
  cycles: bigint;
  lastCycle?: number;
  romLoaded: boolean;
  running: boolean;
  keypad: number;
  canvas?: { context: CanvasRenderingContext2D; imageData: ImageData };
  fps: number;
  paused: boolean;
  instructionCache: InstructionChache;

  run: () => void;
  pause: () => void;
  reset: () => void;
  stepInto: () => void;
  setCanvas: (canvas: HTMLCanvasElement) => void;
  loadRom: (rom: Uint8Array) => void;
  unloadRom: () => void;
  decode: (count: number) => void;

  breakpoints: {
    entries: Set<number>;
    add: (breakPoints: number) => void;
    remove: (breakPoints: number) => void;
  };

  panel: {
    floating: boolean;
    position: Position;
    toggleFloat: () => void;
    setPosition: (position: Position) => void;
  };
};

export const useDebuggerStore = create<DebuggerStore>((set, get) => ({
  cycles: BigInt(0),
  running: false,
  paused: false,
  romLoaded: false,
  keypad: 0x3ff,
  fps: 0,
  instructionCache: {},
  floatingControls: false,

  panel: {
    floating: false,
    position: "down",

    setPosition: (position) => {
      set((prev) => ({ ...prev, panel: { ...prev.panel, position } }));
    },

    toggleFloat: () =>
      set((prev) => ({
        ...prev,
        panel: { ...prev.panel, floating: !prev.panel.floating },
      })),
  },

  breakpoints: {
    entries: new Set<number>(),

    add: (breakpoint) => {
      set((prev) => ({
        ...prev,
        breakpoints: {
          ...prev.breakpoints,
          entries: prev.breakpoints.entries.add(breakpoint),
        },
      }));
    },

    remove: (breakpoint) => {
      set((prev) => {
        const breakpoints = prev.breakpoints.entries;
        breakpoints.delete(breakpoint);

        return {
          ...prev,
          breakpoints: { ...prev.breakpoints, entries: breakpoints },
        };
      });
    },
  },

  setCanvas: (canvas: HTMLCanvasElement) => {
    const context = canvas.getContext("2d")!;
    const imageData = context.createImageData(240, 160);

    set((prev) => ({ ...prev, canvas: { context, imageData } }));
  },

  loadRom: (rom) => {
    instance.reset();
    instance.loadRom(rom);
    instance.boot();
    set((prev) => ({ ...prev, cycles: instance.cycles(), romLoaded: true }));
  },

  unloadRom: () => {
    set((prev) => ({
      ...prev,
      running: false,
      paused: false,
      romLoaded: false,
    }));
  },

  reset: () => {
    instance.reset();
    instance.boot();
    set((prev) => ({
      ...prev,
      cycles: instance.cycles(),
      instructionCache: {},
      keypad: 0x3ff,
    }));
    get().run();
  },

  pause: () => {
    set((prev) => ({ ...prev, running: false, paused: true }));
  },

  run: () => {
    if (get().running) return;

    set((prev) => ({ ...prev, running: true, paused: false }));

    const frameCounter = new FrameCounter();

    const stepFrame = (ellapsed: number) => {
      const { running, canvas, breakpoints, paused } = get();

      if (!running || paused) {
        return;
      }

      frameCounter.onFrame(ellapsed, {
        interval: 1000,
        callback: (fps) => set((prev) => ({ ...prev, fps })),
      });

      if (breakpoints.entries.size) {
        const cycles = instance.stepFrameWithBreakpoints(
          new Uint32Array(breakpoints.entries.values()),
        );

        set((prev) => ({
          ...prev,
          cycles: instance.cycles(),
          lastCycle: cycles,
        }));

        if (breakpoints.entries.has(instance.execAddress()) || !get().running) {
          return set((prev) => ({
            ...prev,
            cycles: instance.cycles(),
            running: false,
          }));
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

      requestAnimationFrame(stepFrame);
    };

    stepFrame(0);
  },

  stepInto: () => {
    const count = instance.debugSyncedStep();

    set((prev) => ({
      ...prev,
      lastCycle: count,
      cycles: prev.cycles + BigInt(count),
    }));
  },

  decode: (count) => {
    const size = instance.instructionSize();
    const instructions: [number, string][] = instance.nextInstructions(count);

    set((prev) => ({
      ...prev,
      instructionCache: {
        ...prev.instructionCache,
        ...Object.fromEntries(
          instructions.map(([addr, value]) => [addr, { value, size }]),
        ),
      },
    }));
  },
}));
