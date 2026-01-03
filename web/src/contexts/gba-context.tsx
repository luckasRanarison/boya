import { createContext, useContext, useState } from "react";
import { Gba } from "boya_wasm";

type GbaContext = {
  bios: {
    loaded: boolean;
    load: (value: Uint8Array) => void;
    memory: Uint8Array;
  };
  instance: Gba;
};

const GbaContext = createContext({} as GbaContext);

const instance = new Gba();

function GbaContextProvider(props: React.PropsWithChildren) {
  const [biosLoaded, setBiosLoaded] = useState(false);

  const loadBios = (value: Uint8Array) => {
    instance.loadBios(value);
    setBiosLoaded(true);
  };

  const value = {
    bios: {
      load: loadBios,
      loaded: biosLoaded,
      memory: instance.bios(),
    },
    instance,
  } satisfies GbaContext;

  return (
    <GbaContext.Provider value={value}>{props.children}</GbaContext.Provider>
  );
}

const useGba = () => useContext(GbaContext);

export { useGba, GbaContextProvider };
