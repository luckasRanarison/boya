import { GBA } from "@/lib/gba";
import { create } from "zustand";

type CallStackEntry = {
  caller: number;
  return: number;
};

type DebuggerStore = {
  callstack: CallStackEntry[];

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
    pushStack: (entry: CallStackEntry) => void;
    popStack: () => void;
    clearState: () => void;
  };
};

export const useDebuggerStore = create<DebuggerStore>((set) => ({
  breakpoints: {
    entries: new Set(),
  },

  callstack: [],
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

    pushStack: (entry) => {
      set((prev) => ({ ...prev, callstack: [...prev.callstack, entry] }));
    },

    popStack: () => {
      set((prev) => ({
        ...prev,
        callstack: prev.callstack.filter(
          (_, i) => i !== prev.callstack.length - 1,
        ),
      }));
    },

    clearState: () => {
      set((prev) => ({ ...prev, callstack: [] }));
    },
  },
}));

export const useBreakpoints = () =>
  useDebuggerStore((state) => state.breakpoints).entries;

export const useDebuggerActions = () =>
  useDebuggerStore((state) => state.actions);
