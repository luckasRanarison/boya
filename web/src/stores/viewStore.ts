import type { MemoryViewMode } from "@/components/common/MemoryView";
import { memoryRegions } from "@/lib/gba";
import { formatHex } from "@/utils/format";
import {
  IconArrowsSort,
  IconBlocks,
  IconBug,
  IconCpu,
  IconCube,
  IconDatabase,
  IconFileDigit,
  IconInfoCircle,
  IconLayoutDashboard,
  IconPalette,
  IconPhoto,
  IconSettings,
  IconSourceCode,
  IconStack2,
  IconStack3,
} from "@tabler/icons-react";
import { create } from "zustand";

export const views = [
  { name: "main", icon: IconLayoutDashboard },
  {
    name: "memory",
    icon: IconStack3,
    sub: [
      { name: "bios", icon: IconFileDigit },
      { name: "ewram", icon: IconStack3 },
      { name: "iwram", icon: IconStack2 },
      { name: "palette", icon: IconPalette },
      { name: "vram", icon: IconPhoto },
      { name: "oam", icon: IconCube },
      { name: "rom", icon: IconSourceCode },
      { name: "sram", icon: IconDatabase },
    ],
  },
  {
    name: "registers",
    icon: IconArrowsSort,
    sub: [
      { name: "cpu", icon: IconCpu },
      { name: "i/o", icon: IconBlocks },
    ],
  },
  { name: "about", icon: IconInfoCircle, mobileOnly: true },
  { name: "debugger", icon: IconBug, mobileOnly: true },
  { name: "settings", icon: IconSettings, mobileOnly: true },
] as const;

type MenuView = {
  name: (typeof views)[number]["name"];
  sub?: {
    name: string;
    metadata?: unknown;
  };
};

type ViewStore = {
  view: MenuView;
  setView: (view: MenuView) => void;
  gotoMemory: (address: number, mode: MemoryViewMode) => void;
};

export const useView = create<ViewStore>((set, get) => ({
  view: {
    name: "main",
  },

  setView: (view: MenuView) => set((prev) => ({ ...prev, view })),

  gotoMemory: (address, mode) => {
    const region = Object.entries(memoryRegions).find(
      ([, data]) => address < data.offset + data.length,
    );

    if (!region) return;

    get().setView({
      name: "memory",
      sub: {
        name: region[0],
        metadata: { mode, address },
      },
    });

    const id = formatHex(address);
    const elem = document.getElementById(id);

    if (!elem) {
      const link = document.createElement("a");
      link.href = `#${id}`;
      link.click();
      link.remove();
    }

    elem?.scrollIntoView({ block: "center", behavior: "smooth" });
  },
}));
