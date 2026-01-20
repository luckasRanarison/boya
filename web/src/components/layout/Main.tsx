import type { MemoryRegion } from "@/lib/gba";
import MemoryView from "../common/MemoryView";
import type { MenuView } from "../views";
import DebuggerView from "../views/debugger/DebuggerView";
import MainView from "../views/main/MainView";
import SettingsView from "../views/settings/SettingsView";
import IORegisterView from "../views/io/IORegisterView";

function Main({ view }: { view: MenuView }) {
  if (view.name === "Debugger") return <DebuggerView />;
  if (view.name === "Settings") return <SettingsView />;
  if (view.name === "Registers") return <IORegisterView />;

  if (view.name === "Memory" && view.sub) {
    return <MemoryView region={view.sub as MemoryRegion} />;
  }

  return <MainView />;
}

export default Main;
