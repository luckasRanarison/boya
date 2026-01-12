import { instance, memoryRegions } from "@/lib/gba";
import MemoryView from "../../common/MemoryView";

function RomView() {
  return (
    <MemoryView data={instance.rom()} baseAddress={memoryRegions.rom.offset} />
  );
}

export default RomView;
