import { instance, memoryRegions } from "@/lib/gba";
import MemoryView from "../../common/MemoryView";

function EwramView() {
  return (
    <MemoryView
      data={instance.ewram()}
      baseAddress={memoryRegions.ewram.offset}
    />
  );
}

export default EwramView;
