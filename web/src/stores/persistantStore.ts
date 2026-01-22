import { defaultKeymaps, type Keymap } from "@/lib/keymap";
import { create } from "zustand";
import { persist } from "zustand/middleware";

export type ColorTheme = "light" | "dark";

type PersistantStore = {
  bios?: Uint8Array | null;
  keymap: Keymap;
  theme: ColorTheme;
  decodeDepth: number;

  set<K extends keyof PersistantStore>(key: K, value: PersistantStore[K]): void;
};

export const usePersistantStore = create(
  persist<PersistantStore>(
    (_set) => ({
      bios: null,
      theme: "light",
      keymap: defaultKeymaps,
      decodeDepth: 10,

      set(key, value) {
        _set((prev) => ({ ...prev, [key]: value }));
      },
    }),
    {
      name: "boya_data",
      merge: (unknownState, current) => {
        const state = unknownState as { bios?: Record<string, number> };

        if (state.bios) {
          const values = Object.values<number>(state.bios);
          const bios = new Uint8Array(values);

          return { ...current, ...state, bios } as PersistantStore;
        }

        return current;
      },
    },
  ),
);
