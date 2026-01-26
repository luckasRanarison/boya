import { create } from "zustand";
import { GBA } from "@/lib/gba";
import { FrameCounter } from "@/utils/frame";
import type { Gba } from "boya_wasm";

type RunParams = {
  onFrame: (gba: Gba) => void;
  breakpoints: Set<number>;
};

type RuntimeStore = {
  cycles: bigint;
  lastCycle?: number;
  romLoaded: boolean;
  running: boolean;
  keypad: number;
  fps: number;
  paused: boolean;

  actions: {
    run: (params: RunParams) => void;
    pause: () => void;
    reset: () => void;
    stepInto: () => void;
    load: (rom: Uint8Array) => void;
    unload: () => void;
  };
};

export const useRuntimeStore = create<RuntimeStore>((set, get) => ({
  cycles: BigInt(0),
  running: false,
  paused: false,
  romLoaded: false,
  keypad: 0x3ff,
  fps: 0,
  instructionCache: {},
  floatingControls: false,

  actions: {
    load: (rom) => {
      GBA.reset();
      GBA.loadRom(rom);
      GBA.boot();
      set((prev) => ({ ...prev, cycles: GBA.cycles(), romLoaded: true }));
    },

    unload: () => {
      set((prev) => ({
        ...prev,
        running: false,
        paused: false,
        romLoaded: false,
      }));
    },

    reset: () => {
      GBA.reset();
      GBA.boot();
      set((prev) => ({
        ...prev,
        cycles: GBA.cycles(),
        instructionCache: {},
        keypad: 0x3ff,
      }));
    },

    pause: () => {
      set((prev) => ({ ...prev, running: false, paused: true }));
    },

    run: (params) => {
      if (get().running) return;

      set((prev) => ({ ...prev, running: true, paused: false }));

      const frameCounter = new FrameCounter();
      const breakpoints = new Uint32Array(params.breakpoints.values());

      const stepFrame = (ellapsed: number) => {
        const { running, paused } = get();

        if (!running || paused) return;

        frameCounter.onFrame(ellapsed, {
          interval: 1000,
          callback: (fps) => set((prev) => ({ ...prev, fps })),
        });

        if (params.breakpoints.size) {
          const cycles = GBA.stepFrameWithBreakpoints(breakpoints);

          set((prev) => ({
            ...prev,
            cycles: GBA.cycles(),
            lastCycle: cycles,
          }));

          if (params.breakpoints.has(GBA.execAddress()) || !get().running) {
            return set((prev) => ({
              ...prev,
              cycles: GBA.cycles(),
              running: false,
            }));
          }
        } else {
          GBA.stepFrame();

          set((prev) => ({
            ...prev,
            cycles: GBA.cycles(),
            lastCycle: undefined,
          }));
        }

        params.onFrame(GBA);
        requestAnimationFrame(stepFrame);
      };

      stepFrame(0);
    },

    stepInto: () => {
      const count = GBA.debugSyncedStep();

      set((prev) => ({
        ...prev,
        lastCycle: count,
        cycles: prev.cycles + BigInt(count),
      }));
    },
  },
}));

export const useRuntimeActions = () =>
  useRuntimeStore((state) => state.actions);
