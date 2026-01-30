import { Gba, MemoryRegion, type IOMap } from "boya_wasm";

export const GBA = new Gba();

export const memoryRegions = {
  bios: {
    offset: 0x0000_0000,
    ref: MemoryRegion.BIOS,
  },
  ewram: {
    offset: 0x0200_0000,
    ref: MemoryRegion.EWRAM,
  },
  iwram: {
    offset: 0x0300_0000,
    ref: MemoryRegion.IWRAM,
  },
  io: {
    offset: 0x0400_0000,
    ref: MemoryRegion.IO,
  },
  palette: {
    offset: 0x0500_0000,
    ref: MemoryRegion.PALETTE,
  },
  vram: {
    offset: 0x0600_0000,
    ref: MemoryRegion.VRAM,
  },
  oam: {
    offset: 0x0700_0000,
    ref: MemoryRegion.OAM,
  },
  rom: {
    offset: 0x0800_0000,
    ref: MemoryRegion.ROM,
  },
  sram: {
    offset: 0x0e00_0000,
    ref: MemoryRegion.SRAM,
  },
};

export const psrFlags = {
  N: 1 << 31,
  Z: 1 << 30,
  C: 1 << 29,
  V: 1 << 28,
  I: 1 << 7,
  F: 1 << 6,
  T: 1 << 5,
};

export function getCpuRegistersBank() {
  const psr = GBA.getSpsrBank();

  return [
    {
      registers: GBA.getMainRegisters(),
      psr: GBA.cpsr(),
    },
    {
      label: "fiq",
      registers: GBA.getFiqRegisters(),
      offset: 8,
      psr: psr[0],
    },
    {
      label: "svc",
      registers: GBA.getSvcRegisters(),
      offset: 13,
      psr: psr[1],
    },
    {
      label: "abt",
      registers: GBA.getAbtRegisters(),
      offset: 13,
      psr: psr[2],
    },
    {
      label: "irq",
      registers: GBA.getIrqRegisters(),
      offset: 13,
      psr: psr[3],
    },
    {
      label: "und",
      registers: GBA.getUndRegisters(),
      offset: 13,
      psr: psr[4],
    },
  ];
}

export function getIoRegisters(ioMap: IOMap) {
  const { offset } = memoryRegions.io;

  return ioMap.map((r) => {
    const addr = offset + r.address;
    if (r.size === "Byte") return { ...r, value: GBA.peekByte(addr) };
    if (r.size === "HWord") return { ...r, value: GBA.peekHWord(addr) };
    return { ...r, value: GBA.peekWord(addr) };
  });
}

export function getMemoryRegion(name: MemoryRegionName) {
  const region = memoryRegions[name];

  return {
    offset: region.offset,

    getLength: () => GBA.getRegionLength(region.ref),
    getData: (start: number, end: number) =>
      GBA.getRegionSlice(region.ref, start, end),
  };
}

export type MemoryRegionName = keyof typeof memoryRegions;
export type IORegister = ReturnType<typeof getIoRegisters>;
export type CPURegisterBank = ReturnType<typeof getCpuRegistersBank>;
export type MemoryChunk = { start: number; end: number };
