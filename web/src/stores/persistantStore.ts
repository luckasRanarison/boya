import { create } from "zustand";
import { persist } from "zustand/middleware";

type UploadStore = {
  bios?: Uint8Array | null;
  setBios: (value: Uint8Array) => void;
};

export const usePersistantStore = create(
  persist<UploadStore>(
    (set) => ({
      bios: null,
      setBios: (bios) => set((prev) => ({ ...prev, bios })),
    }),
    {
      name: "boya_data",
      merge: (unknownState, current) => {
        const state = unknownState as { bios?: Record<string, number> };

        if (state.bios) {
          const values = Object.values<number>(state.bios);
          const bios = new Uint8Array(values);

          return { bios } as UploadStore;
        }

        return current;
      },
    },
  ),
);
