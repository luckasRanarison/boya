import {
  GBA,
  getCpuRegistersBank as getCpuRegisterBanks,
  getIoRegisters,
  getMemoryRegion,
} from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { useRuntimeStore } from "@/stores/runtimeStore";
import type { IOMap } from "boya_wasm";
import { useMemo } from "react";

export type GbaState = ReturnType<typeof useGba>;
export type CpuState = GbaState["cpu"];
export type InstructionPipeline = ReturnType<CpuState["pipeline"]>;

export function useGba() {
  const rom = useRuntimeStore((state) => state.rom);
  const cycles = useRuntimeStore((state) => state.cycles);
  const instructionCache = useDebuggerStore((state) => state.instructionCache);

  const ioMap: IOMap = useMemo(() => GBA.generateIOMap(), []);
  const pc = rom ? GBA.execAddress() : 0;
  const nextPc = pc + GBA.instructionSize();

  // Accessing SP, LR, or operating mode before boot causes a panic
  return {
    cpu: {
      pc,
      lr: rom ? GBA.lr() : 0,
      sp: rom ? GBA.sp() : 0,
      operatingMode: rom ? GBA.cpuOperatingMode() : undefined,
      getRegisters: getCpuRegisterBanks,

      pipeline: () =>
        [
          { address: pc, value: instructionCache[pc]?.value },
          { address: nextPc, value: instructionCache[nextPc]?.value },
        ] as const,
    },

    memory: {
      getIoRegisters: () => getIoRegisters(ioMap),
      getRegion: getMemoryRegion,
      getPalette: GBA.colorPalette.bind(GBA),
      getObjects: GBA.objects.bind(GBA),
    },

    cycles,
    scanline: GBA.scanline(),
    booted: rom,
  };
}
