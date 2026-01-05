import { instance, memoryRegions } from "../../../lib/gba";
import ByteArray from "../../ByteArray";

function OamView() {
  return (
    <ByteArray
      data={instance.oam()}
      baseAddress={memoryRegions.oam.offset}
      pageSize={1024}
    />
  );
}

export default OamView;
