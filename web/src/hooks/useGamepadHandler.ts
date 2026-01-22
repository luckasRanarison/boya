import { GBA } from "@/lib/gba";
import { useDebuggerStore as debuggerStore } from "@/stores/debuggerStore";
import { usePersistantStore } from "@/stores/persistantStore";

export function useGamepadHandler() {
  const { keymap } = usePersistantStore();

  return (event: KeyboardEvent) => {
    const key = keymap[event.code];

    if (!key) return;

    switch (event.type) {
      case "keyup":
        debuggerStore.setState((prev) => ({
          ...prev,
          keypad: prev.keypad | key,
        }));
        break;
      case "keydown":
        debuggerStore.setState((prev) => ({
          ...prev,
          keypad: prev.keypad & ~key,
        }));
        break;
      default:
    }

    GBA.setKeyinput(debuggerStore.getState().keypad);
  };
}
