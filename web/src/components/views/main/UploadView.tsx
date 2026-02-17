import { useRef } from "react";
import {
  Button,
  Text,
  Stack,
  Paper,
  Group,
  Flex,
  ThemeIcon,
  Divider,
  Container,
  Box,
} from "@mantine/core";
import { IconUpload, IconCheck, IconAlertCircle } from "@tabler/icons-react";
import { usePersistantStore } from "@/stores/persistantStore";
import notifications from "@/lib/notifications";
import { useRuntimeActions } from "@/stores/runtimeStore";
import IconGBA from "@/assets/gba.svg?react";

function UploadView() {
  const { bios, theme, set } = usePersistantStore();
  const { load } = useRuntimeActions();

  const romInputRef = useRef<HTMLInputElement>(null);
  const biosInputRef = useRef<HTMLInputElement>(null);

  const handleUpload = (bytes: Uint8Array, type: "rom" | "bios") => {
    if (type === "bios") {
      if (bytes.length !== 0x4000) {
        notifications.error(
          `Invalid BIOS file, expected size: ${0x4000} bytes`,
        );
      } else {
        set("bios", bytes);
        notifications.info(
          "The BIOS file has successfully been uploaded and saved!",
        );
      }
    } else {
      load(bytes);
    }
  };

  const handleButtonUpload = async (params: {
    event: React.ChangeEvent<HTMLInputElement>;
    file: "rom" | "bios";
  }) => {
    const { files } = params.event.target;
    if (files && files.length > 0) {
      const [file] = files;
      const buffer = await file.arrayBuffer();
      const bytes = new Uint8Array(buffer);
      handleUpload(bytes, params.file);
    }
  };

  const handleDropUpload: React.DragEventHandler<HTMLDivElement> = async (
    event,
  ) => {
    event.preventDefault();
    event.currentTarget.style.background = "none";

    const file = event.dataTransfer.files[0];
    if (!file) return;

    const buffer = await file.arrayBuffer();
    const bytes = new Uint8Array(buffer);
    const type = bytes.length === 0x4000 ? "bios" : "rom";

    handleUpload(bytes, type);
  };

  return (
    <Flex p="md" flex={1} align="center" justify="center">
      <Container size="xs" w="100%">
        <Paper
          p="xl"
          radius="lg"
          withBorder
          style={{
            borderStyle: "dashed",
            borderWidth: "2px",
            borderColor: "var(--mantine-color-indigo-light-hover)",
            transition: "all 300ms ease",
          }}
          onDragOver={(event) => {
            event.preventDefault();
            event.currentTarget.style.backgroundColor =
              "var(--mantine-color-indigo-light)";
            event.currentTarget.style.borderColor =
              "var(--mantine-color-indigo-filled)";
          }}
          onDragLeave={(event) => {
            event.currentTarget.style.backgroundColor = "transparent";
            event.currentTarget.style.borderColor =
              "var(--mantine-color-indigo-light-hover)";
          }}
          onDrop={handleDropUpload}
        >
          <Stack align="center">
            <Stack w="100%" gap="xl" align="center">
              <Box mt="md" c="indigo">
                <IconGBA scale={2} />
              </Box>

              <Text c="dimmed" ta="center" size="sm" maw={300}>
                {bios
                  ? "Select a GameBoy Advance ROM (.gba) to get started."
                  : "Welcome! You'll need to upload a BIOS file (.bin) to begin."}
              </Text>
            </Stack>

            <Stack w="100%">
              <Button
                disabled={!bios}
                leftSection={<IconUpload size={20} />}
                onClick={() => romInputRef.current?.click()}
                variant={theme === "dark" ? "light" : "filled"}
                color="indigo"
              >
                Load Game ROM
              </Button>

              <Divider
                label="System Status"
                labelPosition="center"
                my="sm"
                variant="dashed"
              />

              <Group justify="space-between" wrap="nowrap">
                <Group gap="xs">
                  {bios ? (
                    <ThemeIcon color="teal" size="sm" radius="xl">
                      <IconCheck size={12} />
                    </ThemeIcon>
                  ) : (
                    <ThemeIcon color="orange" size="sm" radius="xl">
                      <IconAlertCircle size={12} />
                    </ThemeIcon>
                  )}
                  <Text size="sm">BIOS File</Text>
                  {!bios && (
                    <Text c="orange" size="sm">
                      (required)
                    </Text>
                  )}
                </Group>

                <Button
                  variant="light"
                  size="compact-sm"
                  color={bios ? "gray" : "indigo"}
                  onClick={() => biosInputRef.current?.click()}
                >
                  {bios ? "Replace" : "Upload"}
                </Button>
              </Group>
            </Stack>

            <Text visibleFrom="sm" size="xs" c="dimmed" ta="center">
              Pro tip: You can drag and drop files anywhere on this card.
            </Text>
          </Stack>
        </Paper>
      </Container>

      <div style={{ display: "none" }}>
        <input
          type="file"
          ref={romInputRef}
          accept=".gba,.bin"
          onChange={(event) => handleButtonUpload({ event, file: "rom" })}
        />
        <input
          type="file"
          ref={biosInputRef}
          accept=".bin"
          onChange={(event) => handleButtonUpload({ event, file: "bios" })}
        />
      </div>
    </Flex>
  );
}

export default UploadView;
