import { ActionIcon, Group, Stack, Text } from "@mantine/core";
import { instance } from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex, parseHex } from "@/utils";
import { IconExternalLink } from "@tabler/icons-react";
import { useView } from "@/stores/viewStore";

function DebuggerStatus() {
  const { cycles, lastCycle, romLoaded } = useDebuggerStore();
  const { gotoMemory } = useView();

  // Accessing SP, LR, or operating mode before boot causes a panic
  const rows = [
    {
      label: "PC",
      default: formatHex(0),
      link: true,
      value: () => formatHex(instance.execAddress()),
    },
    {
      label: "LR",
      default: formatHex(0),
      link: true,
      value: () => formatHex(instance.lr()),
    },
    {
      label: "SP",
      default: formatHex(0),
      link: true,
      value: () => formatHex(instance.sp()),
    },
    {
      label: "Mode",
      default: "none",
      value: () => instance.cpuOperatingMode(),
    },
    {
      label: "Cycles",
      default: 0,
      extra: lastCycle && <Text c="green">(+ {lastCycle})</Text>,
      value: () => cycles,
    },
  ];

  return (
    <Stack p="md">
      {rows.map((row) => (
        <Group key={row.label} justify="space-between">
          <Group>
            <Text size="sm">{row.label}:</Text>
            {romLoaded ? (
              <Text c="indigo">{row.value()}</Text>
            ) : (
              <Text c="gray">{row.default}</Text>
            )}
            {row.extra}
          </Group>
          {row.link && (
            <ActionIcon
              variant="subtle"
              onClick={() => gotoMemory(parseHex(row.value()), "code")}
            >
              <IconExternalLink size={18} />
            </ActionIcon>
          )}
        </Group>
      ))}
    </Stack>
  );
}

export default DebuggerStatus;
