import { create } from "zustand";
import { persist } from "zustand/middleware";

export type ColorTheme = "light" | "dark";

type UploadStore = {
  bios?: Uint8Array | null;
  theme: ColorTheme;
  setBios: (value: Uint8Array) => void;
  setTheme: (theme: ColorTheme) => void;
};

export const usePersistantStore = create(
  persist<UploadStore>(
    (set) => ({
      bios: null,
      theme: "light",
      setBios: (bios) => set((prev) => ({ ...prev, bios })),
      setTheme: (theme) => set((prev) => ({ ...prev, theme })),
    }),
    {
      name: "boya_data",
      merge: (unknownState, current) => {
        const state = unknownState as { bios?: Record<string, number> };

        if (state.bios) {
          const values = Object.values<number>(state.bios);
          const bios = new Uint8Array(values);

          return { ...current, ...state, bios } as UploadStore;
        }

        return current;
      },
    },
  ),
);
