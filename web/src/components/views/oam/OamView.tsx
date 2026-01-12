import { instance, memoryRegions } from "@/lib/gba";
import MemoryView from "../../common/MemoryView";

function OamView() {
  return (
    <MemoryView data={instance.oam()} baseAddress={memoryRegions.oam.offset} />
  );
}

export default OamView;
