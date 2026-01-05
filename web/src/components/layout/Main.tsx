import UploadArea from "../UploadArea";
import BiosView from "../views/BiosView";
import DebuggerView from "../views/DebuggerView";
import type { View } from "../../App";

function Main(props: { view: View }) {
  if (props.view === "bios") return <BiosView />;
  if (props.view === "debugger") return <DebuggerView />;

  return <UploadArea />;
}

export default Main;
