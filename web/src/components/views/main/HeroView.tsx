import { useRuntimeStore } from "@/stores/runtimeStore";
import EmulatorView from "./EmulatorView";
import UploadView from "./UploadView";

function HeroView() {
  const romLoaded = useRuntimeStore((state) => state.romLoaded);

  if (romLoaded) {
    return <EmulatorView />;
  } else {
    return <UploadView />;
  }
}

export default HeroView;
