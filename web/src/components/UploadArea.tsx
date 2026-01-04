import { useRef } from "react";
import { Button, Text, Stack, Mark, Paper, Group, Flex } from "@mantine/core";
import {
  IconAlertTriangle,
  IconDragDrop,
  IconUpload,
} from "@tabler/icons-react";
import { usePersistantStore } from "../stores/persistantStore";
import notifications from "../lib/notifications";

function UploadArea() {
  const { bios, setBios } = usePersistantStore();

  const romInputRef = useRef<HTMLInputElement>(null);
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
      if (bytes.length !== 0x4000) {
        notifications.error(
          `Invalid BIOS file, expected size: ${0x4000} bytes`,
        );
      } else {
        setBios(bytes);
      }
    } else {
      // instance.loadRom(bytes);
    }
  };

  return (
    <Flex p="xl" flex={1} align="center" justify="center">
      <Paper
        p="xl"
        w="100%"
        h={{ base: "100%", md: "80%" }}
        maw={{ base: "100%", sm: "65%" }}
        bd="2px dashed indigo"
        radius="md"
      >
        <Stack h="100%" justify="center" align="center">
          <Text c="indigo">
            <IconDragDrop size={70} />
          </Text>
          <Text ta="center">
            Drag and drop your{" "}
            <Mark bg="none" c="indigo" fw={600}>
              GameBoy Advance
            </Mark>{" "}
            files here
          </Text>
          <Text>OR</Text>

          <Group>
            <Button
              w={{ base: "100%", md: "auto" }}
              disabled={!bios}
              leftSection={<IconUpload size={14} />}
              onClick={() => romInputRef.current?.click()}
            >
              Upload ROM
            </Button>
            <Button
              variant="light"
              w={{ base: "100%", md: "auto" }}
              leftSection={<IconUpload size={14} />}
              onClick={() => biosInputRef.current?.click()}
            >
              Upload BIOS
            </Button>
          </Group>

          {!bios && (
            <Stack mt="sm" c="red" align="center" gap="xs">
              <IconAlertTriangle size={20} />
              <Text>BIOS is required to run a game.</Text>
            </Stack>
          )}
        </Stack>

        <div hidden>
          <input
            type="file"
            ref={romInputRef}
            onChange={(event) => handleUpload({ event, file: "rom" })}
          />
          <input
            type="file"
            ref={biosInputRef}
            onChange={(event) => handleUpload({ event, file: "bios" })}
          />
        </div>
      </Paper>
    </Flex>
  );
}

export default UploadArea;
