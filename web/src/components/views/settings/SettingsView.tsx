import { Group, Stack, Text, NumberInput, Kbd, Divider } from "@mantine/core";
import { usePersistantStore } from "../../../stores/persistantStore";
import {
  formatKeyAction,
  formatGamepadKey,
  encodeKeyEvent,
} from "@/lib/keymap";
import { useState, Fragment, useEffect } from "react";
import SettingsCard from "./SettingsCard";
import SettingsSwitch from "./SettingsSwitch";

const modifiers = ["Shift", "Alt", "Control"];

function SettingsView() {
  const settings = usePersistantStore();
  const [keyEditId, setKeyEditId] = useState<number | null>(null);

  useEffect(() => {
    if (!keyEditId) return;

    const handler = (event: KeyboardEvent) => {
      event.preventDefault();
      const encoded = encodeKeyEvent(event);

      if (!modifiers.includes(event.key) && !settings.keymap[encoded]) {
        const newKeymap = Object.entries(settings.keymap).map(
          ([key, value], id) =>
            keyEditId === id ? [encoded, value] : [key, value],
        );
        settings.set("keymap", Object.fromEntries(newKeymap));

        document.removeEventListener("keydown", handler);
        setKeyEditId(null);
      }
    };

    document.addEventListener("keydown", handler);

    return () => document.removeEventListener("keydown", handler);
  }, [keyEditId, settings]);

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
      <SettingsCard title="Interface">
        <SettingsSwitch
          label="Dark mode"
          checked={settings.theme === "dark"}
          onChange={() =>
            settings.set("theme", settings.theme === "dark" ? "light" : "dark")
          }
        />
      </SettingsCard>

      <SettingsCard title="Emulation">
        <SettingsSwitch
          label="Skip BIOS"
          description="Skip boot animation"
          disabled={true} // TODO: implementation
          checked={settings.skipBios}
          onChange={() => settings.set("skipBios", !settings.skipBios)}
        />

        <SettingsSwitch
          label="Cycle accuracy"
          description="Performance can be improved if disabled, but it might break some games."
          disabled={true} // TODO: implementation
          checked={settings.cycleAccuracy}
          onChange={() =>
            settings.set("cycleAccuracy", !settings.cycleAccuracy)
          }
        />
      </SettingsCard>

      <SettingsCard title="Graphics">
        <SettingsSwitch
          label="Smooth filter"
          description="Enable anti-aliasing for rendering"
          checked={settings.smoothFilter}
          onChange={() => settings.set("smoothFilter", !settings.smoothFilter)}
        />
      </SettingsCard>

      <SettingsCard title="Gamepad / Controls">
        <Stack>
          <SettingsSwitch
            label="Display keystrokes"
            description="Show pressed buttons in the footer"
            checked={settings.debugKeys}
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
                  onClick={() => setKeyEditId(keyEditId === id ? null : id)}
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
