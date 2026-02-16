import type { MemoryViewMode } from "@/components/views/memory/MemoryView";
import type { Position } from "@/hooks/useFloatingPositions";
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
import type { Gba } from "boya_wasm";
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
  { name: "objects", icon: IconCube },
  { name: "backgrounds", icon: IconPhoto },
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
  floatingWindows: string[];
  debugPanelPosition: Position;

  canvas?: {
    context: CanvasRenderingContext2D;
    imageData: ImageData;
  };

  actions: {
    setView: (view: MenuView) => void;
    setTab: (tab: NavbarTab) => void;
    setCanvas: (canvas: HTMLCanvasElement) => void;
    renderFrame: (gba: Gba) => void;
    toggleWindow: (name: string) => void;
    moveDebugPanel: (position: Position) => void;
  };
};

export const useViewStore = create<ViewStore>((set, get) => ({
  view: {
    name: "main",
  },

  tab: "about",
  debugPanelPosition: "down",
  floatingWindows: [],

  actions: {
    setView: (view) => set((prev) => ({ ...prev, view })),
    setTab: (tab) => set((prev) => ({ ...prev, tab })),

    setCanvas: (canvas: HTMLCanvasElement) => {
      const context = canvas.getContext("2d")!;
      const imageData = context.createImageData(240, 160);

      set((prev) => ({ ...prev, canvas: { context, imageData } }));
    },

    renderFrame: (gba) => {
      const { canvas } = get();

      if (canvas) {
        const pixels = canvas.imageData.data;
        gba.writeFrameBuffer(pixels as unknown as Uint8Array);
        canvas.context.putImageData(canvas.imageData, 0, 0);
      }
    },

    moveDebugPanel: (position) => {
      set((prev) => ({
        ...prev,
        debugPanelPosition: position,
      }));
    },

    toggleWindow: (key: string) => {
      set((prev) => ({
        ...prev,
        floatingWindows: prev.floatingWindows.includes(key)
          ? prev.floatingWindows.filter((k) => k !== key)
          : [...prev.floatingWindows, key],
      }));
    },
  },
}));

export const useViewActions = () => useViewStore((state) => state.actions);
