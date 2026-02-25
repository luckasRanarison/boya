import {
  useBreakpoints,
  useDebuggerActions,
  useDebuggerStore,
} from "@/stores/debuggerStore";
import { useViewActions } from "@/stores/viewStore";
import { useGotoMemory } from "./useGotoMemory";
import { useRuntimeActions, useRuntimeStore } from "@/stores/runtimeStore";
import { GBA } from "@/lib/gba";
import { useActiveRoute } from "./useActiveRoute";

export function useDebuggerControls() {
  const breakpoints = useBreakpoints();
  const { parent } = useActiveRoute();
  const rom = useRuntimeStore((state) => state.rom);
  const callstack = useDebuggerStore((state) => state.callstack);
  const running = useRuntimeStore((state) => state.running);

  const rt = useRuntimeActions();
  const gotoMemory = useGotoMemory();
  const { renderFrame } = useViewActions();
  const { clearState } = useDebuggerActions();

  const jumpToExec = () => {
    if (parent === "memory") {
      gotoMemory({
        address: GBA.execAddress(),
        mode: "code",
      });
    }
  };

  const reset = () => {
    if (rom) {
      rt.reset();
      clearState();
      rt.run({ onFrame: renderFrame, hooks: { breakpoints } });
    }
  };

  const stepInto = () => {
    if (!running && rom) {
      rt.step({ type: "into" });
      jumpToExec();
    }
  };

  const stepOut = () => {
    const entry = callstack[callstack.length - 1];
    if (entry && !running && rom) {
      rt.run({
        onFrame: renderFrame,
        hooks: { breakpoints: new Set([entry.return]) },
      });
      jumpToExec();
    }
  };

  const stepScanline = () => {
    if (!running && rom) {
      rt.step({ type: "scanline" });
    }
  };

  const stepFrame = () => {
    if (!running && rom) {
      rt.step({ type: "frame" });
    }
  };

  const stepIrq = () => {
    if (!running && rom) {
      rt.run({ onFrame: renderFrame, hooks: { breakpoints, irq: true } });
    }
  };

  const toggleRun = () => {
    if (running) {
      rt.pause();
    } else {
      rt.run({ onFrame: renderFrame, hooks: { breakpoints } });
    }
  };

  return {
    reset,
    stepInto,
    stepOut,
    stepScanline,
    stepFrame,
    stepIrq,
    toggleRun,
    stop: rt.unload,
  };
}
