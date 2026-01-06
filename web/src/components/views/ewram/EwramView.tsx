import { instance, memoryRegions } from "@/lib/gba";
import ByteArray from "../../common/ByteArray";

function EwramView() {
  return (
    <ByteArray
      data={instance.ewram()}
      baseAddress={memoryRegions.ewram.offset}
      pageSize={1024}
    />
  );
}

export default EwramView;
