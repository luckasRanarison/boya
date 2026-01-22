import { Gba, type IOMap } from "boya_wasm";

export const GBA = new Gba();

export const memoryRegions = {
  bios: {
    offset: 0x0000_0000,
    length: 0x4000,
    getData: () => GBA.bios(),
  },
  ewram: {
    offset: 0x0200_0000,
    length: 0x40000,
    getData: () => GBA.ewram(),
  },
  iwram: {
    offset: 0x0300_0000,
    length: 0x8000,
    getData: () => GBA.iwram(),
  },
  io: {
    offset: 0x0400_0000,
    length: 0x210,
    getData: () => new Uint8Array(),
  },
  palette: {
    offset: 0x0500_0000,
    length: 0x400,
    getData: () => GBA.palette(),
  },
  vram: {
    offset: 0x0600_0000,
    length: 0x18000,
    getData: () => GBA.vram(),
  },
  oam: {
    offset: 0x0700_0000,
    length: 0x400,
    getData: () => GBA.oam(),
  },
  rom: {
    offset: 0x0800_0000,
    length: 0xffff_ffff,
    getData: () => GBA.rom(),
  },
  sram: {
    offset: 0x0e00_0000,
    length: 0x1000,
    getData: () => GBA.sram(),
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
  return ioMap.map((r) =>
    r.size === "HWord"
      ? { ...r, value: GBA.readHWord(r.address) }
      : { ...r, value: GBA.readWord(r.address) },
  );
}

export function getMemoryRegion(name: MemoryRegion) {
  const region = memoryRegions[name];

  return {
    lenght: region.length,
    offset: region.offset,
    data: region.getData(),
  };
}

export type MemoryRegion = keyof typeof memoryRegions;
export type IORegister = ReturnType<typeof getIoRegisters>;
export type CPURegisterBank = ReturnType<typeof getCpuRegistersBank>;
