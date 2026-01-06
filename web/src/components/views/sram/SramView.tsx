import { instance, memoryRegions } from "@/lib/gba";
import ByteArray from "../../common/ByteArray";

function SramView() {
  return (
    <ByteArray
      data={instance.sram()}
      baseAddress={memoryRegions.sram.offset}
      pageSize={1024}
    />
  );
}

export default SramView;
