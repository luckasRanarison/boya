import { instance, memoryRegions } from "../../../lib/gba";
import ByteArray from "../../ByteArray";

function VramView() {
  return (
    <ByteArray
      data={instance.vram()}
      baseAddress={memoryRegions.vram.offset}
      pageSize={1024}
    />
  );
}

export default VramView;
