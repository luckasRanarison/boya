import { defaultKeymaps, type Keymap } from "@/lib/keymap";
import { create } from "zustand";
import { persist } from "zustand/middleware";

export type ColorTheme = "light" | "dark";

type PersistantStore = {
  version?: string;
  bios?: Uint8Array | null;
  keymap: Keymap;
  theme: ColorTheme;
  decodeDepth: number;
  smoothFilter: boolean;
  debugKeys: boolean;

  set<K extends keyof PersistantStore>(key: K, value: PersistantStore[K]): void;
};

export const usePersistantStore = create(
  persist<PersistantStore>(
    (_set) => ({
      version: "1.0",
      bios: null,
      theme: "light",
      keymap: defaultKeymaps,
      decodeDepth: 10,
      smoothFilter: false,
      debugKeys: true,

      set(key, value) {
        _set((prev) => ({ ...prev, [key]: value }));
      },
    }),
    {
      name: "boya_data",
      merge: (unknownState, current) => {
        const state = unknownState as Partial<PersistantStore>;

        migrate(state, current);

        if (state.bios) {
          const values = Object.values<number>(state.bios);
          const bios = new Uint8Array(values);
          state.bios = bios;
        }

        return { ...current, ...state };
      },
    },
  ),
);

function migrate(state: Partial<PersistantStore>, current: PersistantStore) {
  if (!state.version) {
    state.keymap = current.keymap;
  }
}
