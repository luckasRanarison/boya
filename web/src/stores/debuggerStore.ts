import { create } from "zustand";
import { instance } from "../lib/gba";

type DebuggerStore = {
  cycles: bigint;
  lastCycle?: number;
  stepInto: () => void;
};

export const useDebuggerStore = create<DebuggerStore>((set, get) => ({
  cycles: BigInt(0),

  stepInto: () => {
    if (get().cycles === BigInt(0)) {
      instance.boot(); // FIXME: uhm
    }

    const count = instance.debugSyncedStep();

    set((prev) => ({
      ...prev,
      lastCycle: count,
      cycles: instance.cycles(),
    }));
  },
}));
