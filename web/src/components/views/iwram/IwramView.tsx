import { instance, memoryRegions } from "@/lib/gba";
import MemoryView from "../../common/MemoryView";

function IwramView() {
  return (
    <MemoryView
      data={instance.iwram()}
      baseAddress={memoryRegions.iwram.offset}
    />
  );
}

export default IwramView;
