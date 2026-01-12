import { instance, memoryRegions } from "@/lib/gba";
import MemoryView from "../../common/MemoryView";

function SramView() {
  return (
    <MemoryView
      data={instance.sram()}
      baseAddress={memoryRegions.sram.offset}
    />
  );
}

export default SramView;
