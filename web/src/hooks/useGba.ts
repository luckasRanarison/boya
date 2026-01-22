import {
  GBA,
  getCpuRegistersBank as getCpuRegisterBanks,
  getIoRegisters,
  getMemoryRegion,
} from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { usePersistantStore } from "@/stores/persistantStore";
import { useView } from "@/stores/viewStore";
import type { IOMap } from "boya_wasm";
import { useEffect, useMemo } from "react";

export type GbaState = ReturnType<typeof useGba>;
export type CpuState = GbaState["cpu"];
export type InstructionPipeline = ReturnType<CpuState["pipeline"]>;

export function useGba() {
  const { view, tab } = useView();
  const { decodeDepth } = usePersistantStore();
  const { cycles, romLoaded, instructionCache, decode } = useDebuggerStore();
  const ioMap: IOMap = useMemo(() => GBA.generateIOMap(), []);

  const decodeStep = useMemo(() => {
    if (view.name === "memory" && view.sub?.metadata?.mode === "code") {
      return decodeDepth;
    } else {
      return view.name === "debugger" || tab === "debugger" ? 2 : 0;
    }
  }, [decodeDepth, tab, view.name, view.sub?.metadata?.mode]);

  const pc = romLoaded ? GBA.execAddress() : 0;
  const nextPc = pc + GBA.instructionSize();

  // revalidate data on cycle change
  useEffect(() => {
    if (decodeStep) {
      decode(decodeStep);
    }
  }, [cycles, decodeStep, decode]);

  // Accessing SP, LR, or operating mode before boot causes a panic
  return {
    cpu: {
      pc,
      lr: romLoaded ? GBA.lr() : 0,
      sp: romLoaded ? GBA.sp() : 0,
      operatingMode: romLoaded ? GBA.cpuOperatingMode() : undefined,
      getRegisters: getCpuRegisterBanks,

      pipeline: () =>
        [
          { address: pc, value: instructionCache[pc]?.value },
          { address: nextPc, value: instructionCache[nextPc]?.value },
        ] as const,
    },

    memory: {
      getRegion: getMemoryRegion,
      getIoRegisters: () => getIoRegisters(ioMap),
    },
  };
}
