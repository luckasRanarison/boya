import { useView } from "@/stores/viewStore";
import type { MemoryRegion } from "@/lib/gba";
import MemoryView, { type MemoryViewMode } from "../common/MemoryView";
import DebuggerView from "../views/debugger/DebuggerView";
import MainView from "../views/main/MainView";
import SettingsView from "../views/settings/SettingsView";
import IORegisterView from "../views/io/IORegisterView";
import CPURegisterView from "../views/cpu/CPURegisterView";

function Main() {
  const { view } = useView();

  if (view.name === "debugger") return <DebuggerView />;
  if (view.name === "settings") return <SettingsView />;

  if (view.name === "memory" && view.sub) {
    const region = view.sub.name as MemoryRegion;
    const metadata = view.sub.metadata as
      | { mode: MemoryViewMode; address: number }
      | undefined;

    return (
      <MemoryView
        region={region}
        mode={metadata?.mode ?? "hex"}
        targetAddress={metadata?.address}
      />
    );
  }

  if (view.name === "registers" && view.sub) {
    if (view.sub.name === "cpu") return <CPURegisterView style="full" />;
    if (view.sub.name === "i/o") return <IORegisterView style="full" />;
  }

  return <MainView />;
}

export default Main;
