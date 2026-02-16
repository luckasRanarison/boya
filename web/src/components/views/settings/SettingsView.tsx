import {
  Group,
  Stack,
  Text,
  NumberInput,
  Switch,
  Kbd,
  Divider,
} from "@mantine/core";
import { usePersistantStore } from "../../../stores/persistantStore";
import {
  formatKeyAction,
  formatGamepadKey,
  encodeKeyEvent,
} from "@/lib/keymap";
import { useState, Fragment } from "react";
import SettingsCard from "./SettingsCard";

const modifiers = ["Shift", "Alt", "Control"];

function SettingsView() {
  const settings = usePersistantStore();
  const [keyEditId, setKeyEditId] = useState<number | null>(null);

  const handleKeyEdit = (targetId: number) => {
    if (keyEditId === targetId) return;

    const handler = (event: KeyboardEvent) => {
      event.preventDefault();
      const encoded = encodeKeyEvent(event);

      if (!modifiers.includes(event.key) && !settings.keymap[encoded]) {
        const newKeymap = Object.entries(settings.keymap).map(
          ([key, value], id) =>
            targetId === id ? [encoded, value] : [key, value],
        );
        settings.set("keymap", Object.fromEntries(newKeymap));

        document.removeEventListener("keydown", handler);
        setKeyEditId(null);
      }
    };

    setKeyEditId(targetId);
    document.addEventListener("keydown", handler);
  };

  return (
    <Stack
      px="md"
      pt="lg"
      pb="20dvh"
      mah="90dvh"
      gap="lg"
      style={{ overflowY: "scroll" }}
      w="100%"
      mx="auto"
    >
      <SettingsCard title="General">
        <Switch
          w="100%"
          label="Dark mode"
          labelPosition="left"
          checked={settings.theme === "dark"}
          styles={{
            body: {
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
            },
          }}
          onChange={() =>
            settings.set("theme", settings.theme === "dark" ? "light" : "dark")
          }
        />
      </SettingsCard>

      <SettingsCard title="Graphics">
        <Switch
          w="100%"
          label="Smooth filter"
          labelPosition="left"
          description="Enable anti-aliasing for rendering"
          checked={settings.smoothFilter}
          styles={{
            body: {
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
            },
          }}
          onChange={() => settings.set("smoothFilter", !settings.smoothFilter)}
        />
      </SettingsCard>

      <SettingsCard title="Gamepad / Controls">
        <Stack>
          <Switch
            w="100%"
            label="Display keystrokes"
            labelPosition="left"
            description="Show preseed buttons in the footer"
            checked={settings.debugKeys}
            styles={{
              body: {
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
              },
            }}
            onChange={() => settings.set("debugKeys", !settings.debugKeys)}
          />
          <Divider />
          <Stack gap="sm">
            {Object.entries(settings.keymap).map(([key, keymap], id) => (
              <Group key={id} justify="space-between">
                <Text size="sm">
                  {keymap.type === "gamepad"
                    ? formatGamepadKey(keymap.value)
                    : formatKeyAction(keymap.action)}
                </Text>
                <Group
                  gap="xs"
                  onClick={() => handleKeyEdit(id)}
                  style={{ cursor: "pointer" }}
                >
                  {keyEditId === id ? (
                    <Kbd>...</Kbd>
                  ) : (
                    key.split("+").map((part, i, parts) => (
                      <Fragment key={i}>
                        <Kbd>{part.replace(/Key|Arrow|Digit/, "")}</Kbd>
                        {i < parts.length - 1 && (
                          <Text size="xs" c="dimmed">
                            +
                          </Text>
                        )}
                      </Fragment>
                    ))
                  )}
                </Group>
              </Group>
            ))}
          </Stack>
        </Stack>
      </SettingsCard>

      <SettingsCard title="Debugger">
        <NumberInput
          label="Decoding depth"
          description="Adjust decoding depth in code view"
          min={0}
          max={100}
          value={settings.decodeDepth}
          onChange={(n) =>
            settings.set("decodeDepth", typeof n === "string" ? parseInt(n) : n)
          }
        />
      </SettingsCard>
    </Stack>
  );
}

export default SettingsView;
