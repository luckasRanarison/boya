import { create } from "zustand";
import type { MemoryRegion } from "../lib/gba";

export type View = MemoryRegion | "main";

type ViewStore = {
  view: View;
  setView: (value: View) => void;
};

export const useView = create<ViewStore>((set) => ({
  view: "main",

  setView: (currentView: View) =>
    set((prev) => ({ ...prev, view: currentView })),
}));
