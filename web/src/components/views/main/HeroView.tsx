import { useRuntimeStore } from "@/stores/runtimeStore";
import EmulatorView from "./EmulatorView";
import UploadView from "./UploadView";

function HeroView() {
  const romHeader = useRuntimeStore((state) => state.romHeader);

  if (romHeader) {
    return <EmulatorView />;
  } else {
    return <UploadView />;
  }
}

export default HeroView;
