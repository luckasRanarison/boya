import { useDebuggerStore } from "@/stores/debuggerStore";
import UploadArea from "./UploadArea";
import EmulatorView from "./EmulatorView.tsx";

function MainView() {
  const { romLoaded } = useDebuggerStore();

  return romLoaded ? <EmulatorView /> : <UploadArea />;
}

export default MainView;
