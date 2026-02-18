import type { Position } from "@/hooks/useFloatingPositions";
import type { Gba } from "boya_wasm";
import { create } from "zustand";

export type NavbarTab = "about" | "debugger" | "settings";

type ViewStore = {
  tab: NavbarTab;
  floatingWindows: string[];
  debugPanelPosition: Position;

  canvas?: {
    context: CanvasRenderingContext2D;
    imageData: ImageData;
  };

  actions: {
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
