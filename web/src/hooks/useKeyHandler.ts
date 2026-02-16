import { useRuntimeActions } from "@/stores/runtimeStore";
import { usePersistantStore } from "@/stores/persistantStore";
import { controls, encodeKeyEvent } from "@/lib/keymap";
import { useDebuggerControls } from "./useDebuggerControls";

export function useKeyHandler() {
  const keymap = usePersistantStore((state) => state.keymap);
  const dbg = useDebuggerControls();
  const { updateKeypad } = useRuntimeActions();

  return (event: KeyboardEvent) => {
    const mapping = keymap[encodeKeyEvent(event)];

    if (!mapping) return;

    if (mapping && mapping.type === "gamepad") {
      if (event.type === "keyup") {
        updateKeypad((prev) => prev | mapping.value);
      } else if (event.type === "keydown") {
        updateKeypad((prev) => prev & ~mapping.value);
      }

      event.preventDefault();
    } else if (
      !event.repeat &&
      mapping.type === "debugger" &&
      event.type === "keydown"
    ) {
      if (mapping.action === controls.reset) dbg.reset();
      else if (mapping.action === controls.stepInto) dbg.stepInto();
      else if (mapping.action === controls.stepOut) dbg.stepOut();
      else if (mapping.action === controls.stepScanline) dbg.stepScanline();
      else if (mapping.action === controls.stepFrame) dbg.stepFrame();
      else if (mapping.action === controls.stepIRQ) dbg.stepIrq();
      else if (mapping.action === controls.toggleRun) dbg.toggleRun();
      else if (mapping.action === controls.stop) dbg.stop();

      event.preventDefault();
    }
  };
}
