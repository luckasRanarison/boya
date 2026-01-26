import { GBA } from "@/lib/gba";
import { create } from "zustand";

type DebuggerStore = {
  breakpoints: {
    entries: Set<number>;
  };

  instructionCache: {
    [key: number]: { value: string; size: number } | undefined;
  };

  actions: {
    decode: (count: number) => void;
    addBreak: (bp: number) => void;
    removeBreak: (bp: number) => void;
  };
};

export const useDebuggerStore = create<DebuggerStore>((set) => ({
  breakpoints: {
    entries: new Set(),
  },

  instructionCache: {},

  actions: {
    decode: (count) => {
      const size = GBA.instructionSize();
      const instructions: [number, string][] = GBA.nextInstructions(count);

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

    addBreak: (breakpoint) => {
      set((prev) => ({
        ...prev,
        breakpoints: { entries: prev.breakpoints.entries.add(breakpoint) },
      }));
    },

    removeBreak: (breakpoint) => {
      set((prev) => {
        const entries = prev.breakpoints.entries;
        entries.delete(breakpoint);

        return {
          ...prev,
          breakpoints: { entries },
        };
      });
    },
  },
}));

export const useBreakpoints = () =>
  useDebuggerStore((state) => state.breakpoints).entries;

export const useDebuggerActions = () =>
  useDebuggerStore((state) => state.actions);
