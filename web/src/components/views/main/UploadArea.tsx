import { useRef } from "react";
import { Button, Text, Stack, Mark, Paper, Group, Flex } from "@mantine/core";
import { IconDragDrop, IconUpload } from "@tabler/icons-react";
import { usePersistantStore } from "@/stores/persistantStore";
import notifications from "@/lib/notifications";
import { useDebuggerStore } from "@/stores/debuggerStore";

function UploadArea() {
  const { bios, theme, setBios } = usePersistantStore();
  const { loadRom } = useDebuggerStore();

  const romInputRef = useRef<HTMLInputElement>(null);
  const biosInputRef = useRef<HTMLInputElement>(null);

  const handleUpload = (bytes: Uint8Array, type: "rom" | "bios") => {
    if (type === "bios") {
      if (bytes.length !== 0x4000) {
        notifications.error(
          `Invalid BIOS file, expected size: ${0x4000} bytes`,
        );
      } else {
        setBios(bytes);
        notifications.info(
          "The BIOS file has successfully been uploaded and saved to local storage!",
        );
      }
    } else {
      loadRom(bytes);
    }
  };

  const handleButtonUpload = async (params: {
    event: React.ChangeEvent<HTMLInputElement>;
    file: "rom" | "bios";
  }) => {
    const { files } = params.event.target;

    if (files) {
      const [file] = files;
      const bytes = await file.bytes();
      handleUpload(bytes, params.file);
    }
  };

  const handleDropUpload: React.DragEventHandler<HTMLDivElement> = async (
    event,
  ) => {
    event.preventDefault();
    event.currentTarget.style.background = "none";

    const file = event.dataTransfer.files[0];
    const bytes = await file.bytes();
    const type = bytes.length === 0x4000 ? "bios" : "rom";

    handleUpload(bytes, type);
  };

  return (
    <Flex p="xl" flex={1} align="center" justify="center">
      <Paper
        p="xl"
        w="100%"
        h="80%"
        maw={{ base: "100%", sm: "65%" }}
        bd="2px dashed indigo"
        radius="md"
        onDragLeave={({ currentTarget }) => {
          currentTarget.style.background = "none";
        }}
        onDragOver={(event) => {
          event.preventDefault();
          event.currentTarget.style.background =
            theme === "light" ? "#00000008" : "#FFFFFF08";
        }}
        onDrop={handleDropUpload}
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
        </Stack>

        <div hidden>
          <input
            type="file"
            ref={romInputRef}
            onChange={(event) => handleButtonUpload({ event, file: "rom" })}
          />
          <input
            type="file"
            ref={biosInputRef}
            onChange={(event) => handleButtonUpload({ event, file: "bios" })}
          />
        </div>
      </Paper>
    </Flex>
  );
}

export default UploadArea;
