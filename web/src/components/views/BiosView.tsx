import { memoryRegions } from "../../lib/gba";
import { usePersistantStore } from "../../stores/persistantStore";
import ByteArray from "../ByteArray";

function BiosView() {
  const { bios } = usePersistantStore();

  return (
    <ByteArray
      data={bios ?? new Uint8Array(memoryRegions.bios.length)}
      baseAddress={memoryRegions.bios.offset}
      pageSize={1024}
    />
  );
}

export default BiosView;
