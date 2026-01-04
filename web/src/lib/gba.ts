import { Gba } from "boya_wasm";

export const instance = new Gba();

export const memoryRegions = {
  bios: {
    offset: 0x0000_0000,
    length: 0x4000,
  },
  ewram: {
    offset: 0x0200_0000,
    length: 0x40000,
  },
  iwram: {
    offset: 0x0300_0000,
    length: 0x8000,
  },
  palette: {
    offset: 0x0500_0000,
    length: 0x400,
  },
  vram: {
    offset: 0x0600_0000,
    length: 0x18000,
  },
  oam: {
    offset: 0x0700_0000,
    length: 0x400,
  },
};

export type MemoryRegion = keyof typeof memoryRegions;
