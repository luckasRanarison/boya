import { instance, memoryRegions } from "../../../lib/gba";
import ByteArray from "../../ByteArray";

function IwramView() {
  return (
    <ByteArray
      data={instance.iwram()}
      baseAddress={memoryRegions.iwram.offset}
      pageSize={1024}
    />
  );
}

export default IwramView;
