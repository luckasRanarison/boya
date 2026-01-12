import { instance, memoryRegions } from "@/lib/gba";
import ByteArray from "../../common/ByteArray";

function PaletteView() {
  return (
    <ByteArray
      data={instance.palette()}
      baseAddress={memoryRegions.palette.offset}
      pageSize={1024}
      rightSection="color"
    />
  );
}

export default PaletteView;
