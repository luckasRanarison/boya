import { useView } from "@/stores/viewStore";
import type { MemoryRegion } from "@/lib/gba";
import MemoryView from "../views/memory/MemoryView";
import DebuggerView from "../views/debugger/DebuggerView";
import SettingsView from "../views/settings/SettingsView";
import IORegisterView from "../views/io/IORegisterView";
import CPURegisterView from "../views/cpu/CPURegisterView";
import { useDebuggerStore } from "@/stores/debuggerStore";
import EmulatorView from "../views/main/EmulatorView";
import UploadView from "../views/main/UploadView";

function Main() {
  const { view } = useView();
  const { romLoaded } = useDebuggerStore();

  if (view.name === "debugger") return <DebuggerView />;
  if (view.name === "settings") return <SettingsView />;

  if (view.name === "memory" && view.sub) {
    return (
      <MemoryView
        region={view.sub.name as MemoryRegion}
        mode={view.sub.metadata?.mode ?? "hex"}
        targetAddress={view.sub.metadata?.address}
      />
    );
  }

  if (view.name === "registers" && view.sub) {
    if (view.sub.name === "cpu") return <CPURegisterView style="full" />;
    if (view.sub.name === "i/o") return <IORegisterView style="full" />;
  }

  return romLoaded ? <EmulatorView /> : <UploadView />;
}

export default Main;
