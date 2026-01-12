import { instance, memoryRegions } from "@/lib/gba";
import MemoryView from "../../common/MemoryView";

function VramView() {
  return (
    <MemoryView
      data={instance.vram()}
      baseAddress={memoryRegions.vram.offset}
    />
  );
}

export default VramView;
