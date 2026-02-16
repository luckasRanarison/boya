import {
  GBA,
  getCpuRegistersBank as getCpuRegisterBanks,
  getIoRegisters,
  getMemoryRegion,
} from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { useRuntimeStore } from "@/stores/runtimeStore";
import type { IOMap } from "boya_wasm";
import { useEffect, useMemo } from "react";

export type GbaState = ReturnType<typeof useGba>;
export type CpuState = GbaState["cpu"];
export type InstructionPipeline = ReturnType<CpuState["pipeline"]>;

export function useGba() {
  const cycles = useRuntimeStore((state) => state.cycles);
  const romLoaded = useRuntimeStore((state) => state.romLoaded);
  const instructionCache = useDebuggerStore((state) => state.instructionCache);

  const ioMap: IOMap = useMemo(() => GBA.generateIOMap(), []);
  const pc = romLoaded ? GBA.execAddress() : 0;
  const nextPc = pc + GBA.instructionSize();

  useEffect(() => {}, [cycles]);

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
      getIoRegisters: () => getIoRegisters(ioMap),
      getRegion: getMemoryRegion,
      getPalette: GBA.colorPalette.bind(GBA),
      getObjects: GBA.objects.bind(GBA),
    },

    cycles,
    scanline: GBA.scanline(),
    booted: romLoaded,

    renderObj: GBA.renderObjBuffer.bind(GBA),
    renderBg: GBA.renderBgBuffer.bind(GBA),
    renderTile: GBA.renderTileBuffer.bind(GBA),
  };
}
