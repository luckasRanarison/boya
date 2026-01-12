import { memoryRegions } from "@/lib/gba";
import { usePersistantStore } from "@/stores/persistantStore";
import MemoryView from "../../common/MemoryView";

function BiosView() {
  const { bios } = usePersistantStore();

  return (
    <MemoryView
      data={bios ?? new Uint8Array(memoryRegions.bios.length)}
      baseAddress={memoryRegions.bios.offset}
    />
  );
}

export default BiosView;
