import {
  Divider,
  Group,
  Stack,
  Title,
  Text,
  SimpleGrid,
  Button,
  NumberInput,
  Switch,
} from "@mantine/core";
import { usePersistantStore } from "../../../stores/persistantStore";
import { getLabel } from "@/lib/keymap";
import { useState } from "react";

function SettingsView() {
  const settings = usePersistantStore();
  const [keyEditId, setKeyEditId] = useState<number | null>(null);

  const handleKeyEdit = (targetId: number) => {
    if (keyEditId === targetId) return;

    const handler = (event: KeyboardEvent) => {
      if (event.code !== "Escape" && !settings.keymap[event.code]) {
        const newKeymap = Object.entries(settings.keymap).map(
          ([key, value], id) =>
            targetId === id ? [event.code, value] : [key, value],
        );
        settings.set("keymap", Object.fromEntries(newKeymap));
      }

      document.removeEventListener("keydown", handler);
      setKeyEditId(null);
    };

    setKeyEditId(targetId);
    document.addEventListener("keydown", handler);
  };

  return (
    <Stack
      px="md"
      pt="md"
      pb="20dvh"
      mah="90dvh"
      gap="xl"
      style={{ overflow: "scroll" }}
      w={{ base: "100%", md: undefined }}
    >
      <Stack>
        <Title order={4} c="indigo">
          General
        </Title>
        <Divider />
        <Switch
          w="100%"
          label="Dark mode"
          labelPosition="left"
          checked={settings.theme === "dark"}
          styles={{
            body: { display: "flex", justifyContent: "space-between" },
          }}
          onChange={() =>
            settings.set("theme", settings.theme === "dark" ? "light" : "dark")
          }
        />
      </Stack>

      <Stack>
        <Title order={4} c="indigo">
          Gamepad
        </Title>
        <Divider />
        <SimpleGrid>
          {Object.entries(settings.keymap).map(([key, value], id) => (
            <Group key={id} justify="space-between">
              <Text size="sm">{getLabel(value!)}</Text>
              <Button
                variant="outline"
                size="xs"
                onClick={() => handleKeyEdit(id)}
              >
                {keyEditId === id ? "..." : key.replace(/Key|Arrow|Digit/, "")}
              </Button>
            </Group>
          ))}
        </SimpleGrid>
      </Stack>

      <Stack>
        <Title order={4} c="indigo">
          Debugger
        </Title>
        <Divider />

        <NumberInput
          label="Decoding depth"
          min={0}
          max={100}
          w="fit-content"
          value={settings.decodeDepth}
          onChange={(n) =>
            settings.set("decodeDepth", typeof n === "string" ? parseInt(n) : n)
          }
        />
      </Stack>
    </Stack>
  );
}

export default SettingsView;
