import type { View } from "../views";
import BiosView from "../views/bios/BiosView";
import DebuggerView from "../views/debugger/DebuggerView";
import EwramView from "../views/ewram/EwramView";
import IOView from "../views/io/IOView";
import IwramView from "../views/iwram/IwramView";
import MainView from "../views/main/MainView";
import OamView from "../views/oam/OamView";
import PaletteView from "../views/palette/PaletteView";
import RomView from "../views/rom/RomView";
import SettingsView from "../views/settings/SettingsView";
import SramView from "../views/sram/SramView";
import VramView from "../views/vram/VramView";

function Main(props: { view: View }) {
  if (props.view === "bios") return <BiosView />;
  if (props.view === "ewram") return <EwramView />;
  if (props.view === "iwram") return <IwramView />;
  if (props.view === "i/o") return <IOView />;
  if (props.view === "palette") return <PaletteView />;
  if (props.view === "vram") return <VramView />;
  if (props.view === "oam") return <OamView />;
  if (props.view === "rom") return <RomView />;
  if (props.view === "sram") return <SramView />;
  if (props.view === "debugger") return <DebuggerView />;
  if (props.view === "settings") return <SettingsView />;

  return <MainView />;
}

export default Main;
