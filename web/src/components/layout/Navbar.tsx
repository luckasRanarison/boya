import { Button, Group, Stack, Text } from "@mantine/core";
import { IconDeviceGamepad, IconFileDigit } from "@tabler/icons-react";
import { useGba } from "../../contexts/gba-context";
import { useRef } from "react";

function Navbar() {
  const { bios } = useGba();

  const biosInputRef = useRef<HTMLInputElement>(null);

  const handleUpload = async (params: {
    event: React.ChangeEvent<HTMLInputElement>;
    file: "rom" | "bios";
  }) => {
    const { files } = params.event.target;

    if (!files) {
      throw new Error("No files uploaded");
    }

    const [file] = files;
    const bytes = await file.bytes();

    if (params.file === "bios") {
      try {
        bios.load(bytes);
      } catch (error) {
        console.error(error);
      }
    } else {
      // instance.loadRom(bytes);
    }
  };

  return (
    <Stack>
      <Group justify="space-between">
        <Group c={bios.loaded ? "green" : "red"}>
          <IconFileDigit />
          <Text size="sm" fw={600}>
            BIOS
          </Text>
          <input
            type="file"
            ref={biosInputRef}
            onChange={(event) => handleUpload({ event, file: "bios" })}
            hidden
          />
        </Group>
        <Button variant="subtle" onClick={() => biosInputRef.current?.click()}>
          Upload
        </Button>
      </Group>

      <Group justify="space-between">
        <Group>
          <IconDeviceGamepad />
          <Text size="sm" fw={600}>
            ROM
          </Text>
        </Group>
        <Button variant="subtle">Upload</Button>
      </Group>
    </Stack>
  );
}

export default Navbar;
