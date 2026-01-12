import { instance, memoryRegions } from "@/lib/gba";
import MemoryView from "../../common/MemoryView";

function PaletteView() {
  return (
    <MemoryView
      data={instance.palette()}
      baseAddress={memoryRegions.palette.offset}
      rightSection="color"
    />
  );
}

export default PaletteView;
