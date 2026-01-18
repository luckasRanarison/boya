import {
  Divider,
  Group,
  Select,
  Stack,
  Title,
  Text,
  SimpleGrid,
  Button,
} from "@mantine/core";
import {
  usePersistantStore,
  type ColorTheme,
} from "../../../stores/persistantStore";
import { getLabel } from "@/lib/keymap";
import { useState } from "react";

function SettingsView() {
  const { theme, setTheme } = usePersistantStore();
  const { keymap, setKeymap } = usePersistantStore();
  const [keyEditId, setKeyEditId] = useState<number | null>(null);

  const handleKeyEdit = (targetId: number) => {
    if (keyEditId === targetId) return;

    const handler = (event: KeyboardEvent) => {
      if (event.code !== "Escape" && !keymap[event.code]) {
        const newKeymap = Object.entries(keymap).map(([key, value], id) =>
          targetId === id ? [event.code, value] : [key, value],
        );
        setKeymap(Object.fromEntries(newKeymap));
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
        <Select
          label="Theme"
          value={theme}
          onChange={(e) => e && setTheme(e as ColorTheme)}
          data={["light", "dark"]}
        />
      </Stack>

      <Stack>
        <Title order={4} c="indigo">
          Gamepad
        </Title>
        <Divider />
        <SimpleGrid>
          {Object.entries(keymap).map(([key, value], id) => (
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
    </Stack>
  );
}

export default SettingsView;
