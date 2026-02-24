import { useRuntimeStore } from "@/stores/runtimeStore";
import EmulatorView from "./EmulatorView";
import UploadView from "./UploadView";

function HeroView() {
  const rom = useRuntimeStore((state) => state.rom);

  if (rom) {
    return <EmulatorView rom={rom} />;
  } else {
    return <UploadView />;
  }
}

export default HeroView;
