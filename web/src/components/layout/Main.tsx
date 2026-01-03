import { Stack } from "@mantine/core";
import { useGba } from "../../contexts/gba-context";
import ByteArray from "../ByteArray";

function Main() {
  const { bios } = useGba();

  return (
    <Stack justify="center">
      <ByteArray data={bios.memory} baseAddress={0x0000_0000} />
    </Stack>
  );
}

export default Main;
