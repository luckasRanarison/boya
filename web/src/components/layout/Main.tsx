import { useView } from "@/stores/viewStore";
import { useDebuggerStore } from "@/stores/debuggerStore";
import type { MemoryRegion } from "@/lib/gba";
import MemoryView from "../views/memory/MemoryView";
import DebuggerView from "../views/debugger/DebuggerView";
import SettingsView from "../views/settings/SettingsView";
import EmulatorView from "../views/main/EmulatorView";
import UploadView from "../views/main/UploadView";
import RegisterView, {
  type RegisterSubMenu,
} from "../views/registers/RegisterView";

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
        jump={view.sub.metadata?.jump}
      />
    );
  }

  if (view.name === "registers" && view.sub) {
    return <RegisterView sub={view.sub.name as RegisterSubMenu} />;
  }

  return romLoaded ? <EmulatorView /> : <UploadView />;
}

export default Main;
