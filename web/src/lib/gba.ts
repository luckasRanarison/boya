import { Gba } from "boya_wasm";

export const instance = new Gba();

export const memoryRegions = {
  bios: {
    offset: 0x0000_0000,
    length: 0x4000,
    getData: () => instance.bios(),
  },
  ewram: {
    offset: 0x0200_0000,
    length: 0x40000,
    getData: () => instance.ewram(),
  },
  iwram: {
    offset: 0x0300_0000,
    length: 0x8000,
    getData: () => instance.iwram(),
  },
  io: {
    offset: 0x0400_0000,
    getData: () => new Uint8Array(),
  },
  palette: {
    offset: 0x0500_0000,
    length: 0x400,
    getData: () => instance.palette(),
  },
  vram: {
    offset: 0x0600_0000,
    length: 0x18000,
    getData: () => instance.vram(),
  },
  oam: {
    offset: 0x0700_0000,
    length: 0x400,
    getData: () => instance.oam(),
  },
  rom: {
    offset: 0x0800_0000,
    getData: () => instance.rom(),
  },
  sram: {
    offset: 0x0e00_0000,
    length: 0x1000,
    getData: () => instance.sram(),
  },
};

export type MemoryRegion = keyof typeof memoryRegions;

export const psrFlags = {
  N: 1 << 31,
  Z: 1 << 30,
  C: 1 << 29,
  V: 1 << 28,
  I: 1 << 7,
  F: 1 << 6,
  T: 1 << 5,
};

export function getRegistersBank() {
  const psr = instance.getSpsrBank();

  return [
    {
      registers: instance.getMainRegisters(),
      psr: instance.cpsr(),
    },
    {
      label: "fiq",
      registers: instance.getFiqRegisters(),
      offset: 8,
      psr: psr[0],
    },
    {
      label: "svc",
      registers: instance.getSvcRegisters(),
      offset: 13,
      psr: psr[1],
    },
    {
      label: "abt",
      registers: instance.getAbtRegisters(),
      offset: 13,
      psr: psr[2],
    },
    {
      label: "irq",
      registers: instance.getIrqRegisters(),
      offset: 13,
      psr: psr[3],
    },
    {
      label: "und",
      registers: instance.getUndRegisters(),
      offset: 13,
      psr: psr[4],
    },
  ];
}
