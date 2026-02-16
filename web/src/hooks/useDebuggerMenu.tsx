import { useRuntimeStore } from "@/stores/runtimeStore";
import type { GbaState } from "./useGba";
import BreakpointControl from "@/components/views/debugger/BreakpointControl";
import DebuggerStatus from "@/components/views/debugger/DebuggerStatus";
import PipelineView from "@/components/views/debugger/PipelineView";
import CallStack from "@/components/views/debugger/CallStack";
import CPURegisterView from "@/components/views/registers/CPURegisterView";
import IORegisterView from "@/components/views/registers/IORegisterView";

export function useDebuggerMenu(state: GbaState) {
  const running = useRuntimeStore((state) => state.running);

  return [
    {
      key: "breakpoint",
      label: "Breakpoints",
      view: <BreakpointControl disabled={running} />,
    },
    {
      key: "status",
      label: "Status",
      view: <DebuggerStatus state={state} running={running} />,
    },
    {
      key: "pipeline",
      label: "Pipeline",
      view: <PipelineView cpu={state.cpu} />,
    },
    {
      key: "callstack",
      label: "Call Stack",
      view: <CallStack disabled={running} />,
    },
    {
      key: "cpu_reg",
      label: "CPU Registers",
      view: <CPURegisterView value={state.cpu.getRegisters()} style="simple" />,
    },
    {
      key: "io_reg",
      label: "I/O Registers",
      view: (
        <IORegisterView value={state.memory.getIoRegisters()} style="simple" />
      ),
    },
  ];
}
