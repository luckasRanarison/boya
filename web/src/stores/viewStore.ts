import type { MemoryViewMode } from "@/components/views/memory/MemoryView";
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

type ViewName = (typeof views)[number]["name"];

type ViewMetadata = {
  memory: {
    mode?: MemoryViewMode;
    jump?: { address: number };
  };
};

export type MenuView<K extends string = ViewName> = {
  name: K;
  sub?: {
    name: string;
    metadata?: K extends keyof ViewMetadata ? ViewMetadata[K] : never;
  };
};

export type NavbarTab = "about" | "debugger" | "settings";

type ViewStore = {
  view: MenuView;
  tab: NavbarTab;

  setView: (view: MenuView) => void;
  setTab: (tab: NavbarTab) => void;
};

export const useView = create<ViewStore>((set) => ({
  view: {
    name: "main",
  },

  tab: "about",

  setView: (view) => set((prev) => ({ ...prev, view })),
  setTab: (tab) => set((prev) => ({ ...prev, tab })),
}));
