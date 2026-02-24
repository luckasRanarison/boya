import { create } from "zustand";
import { GBA } from "@/lib/gba";
import { FrameCounter } from "@/utils/frame";
import type { CartridgeHeader, Gba } from "boya_wasm";

export type RomData = {
  header: CartridgeHeader;
  metadata: {
    size: number;
  };
};

type StepKind = "into" | "scanline" | "frame";

type RunParams = {
  onFrame: (gba: Gba) => void;
  breakpoints?: Set<number>;
  irq?: boolean;
};

type RuntimeStore = {
  cycles: bigint;
  lastCycle?: number;
  running: boolean;
  rom?: RomData;
  keypad: number;
  fps: number;
  paused: boolean;

  actions: {
    run: (params: RunParams) => void;
    pause: () => void;
    reset: () => void;
    step: (params: { type: StepKind }) => void;
    load: (rom: Uint8Array) => void;
    unload: () => void;
    updateKeypad: (f: (value: number) => number) => void;
  };
};

export const useRuntimeStore = create<RuntimeStore>((set, get) => {
  const updateCycles = () => {
    set((prev) => ({
      ...prev,
      cycles: GBA.cycles(),
      lastCycle: Number(GBA.cycles() - prev.cycles),
    }));
  };

  return {
    cycles: BigInt(0),
    running: false,
    paused: false,
    keypad: 0x3ff,
    fps: 0,
    instructionCache: {},
    floatingControls: false,

    actions: {
      load: (rom) => {
        const header: CartridgeHeader = GBA.parseHeader(rom);

        GBA.reset();
        GBA.loadRom(rom);
        GBA.boot();

        set((prev) => ({
          ...prev,
          cycles: GBA.cycles(),
          rom: {
            header,
            metadata: {
              size: rom.length,
            },
          },
        }));
      },

      unload: () => {
        set((prev) => ({
          ...prev,
          running: false,
          paused: false,
          rom: undefined,
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
        const breakpoints = new Uint32Array(params.breakpoints?.values() || []);

        const stepFrame = (ellapsed: number) => {
          const { running, paused } = get();

          if (!running || paused) return;

          let halt = false;

          frameCounter.onFrame(ellapsed, {
            interval: 1000,
            callback: (fps) => set((prev) => ({ ...prev, fps })),
          });

          if (breakpoints.length || params.irq) {
            halt = GBA.stepFrameWithHooks(breakpoints, params.irq ?? false);
          } else {
            GBA.stepFrame();
          }

          updateCycles();

          params.onFrame(GBA);

          if (halt) {
            set((prev) => ({ ...prev, running: false }));
          } else {
            requestAnimationFrame(stepFrame);
          }
        };

        stepFrame(0);
      },

      step: (params) => {
        if (params.type === "frame") GBA.stepFrame();
        if (params.type === "scanline") GBA.stepScanline();
        if (params.type === "into") GBA.debugSyncedStep();

        updateCycles();
      },

      updateKeypad: (f) => {
        set((prev) => {
          const value = f(prev.keypad);
          GBA.setKeyinput(value);
          return { ...prev, keypad: value };
        });
      },
    },
  };
});

export const useRuntimeActions = () =>
  useRuntimeStore((state) => state.actions);
