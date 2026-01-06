import { Group, Stack, Text } from "@mantine/core";
import { instance } from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils";

function DebuggerStatus() {
  const { cycles, lastCycle } = useDebuggerStore();
  const hasState = cycles > 0;

  const rows = [
    { label: "PC", value: formatHex(instance.pc()) },
    { label: "LR", value: formatHex(hasState ? instance.lr() : 0) },
    { label: "SP", value: formatHex(hasState ? instance.sp() : 0) },
    {
      label: "Mode",
      value: hasState ? instance.cpuOperatingMode() : "none",
      color: hasState ? "indigo" : "gray",
    },
    {
      label: "Cycles",
      value: cycles,
      extra: lastCycle && <Text c="green">(+ {lastCycle})</Text>,
    },
  ];
  return (
    <Stack mb="md">
      {rows.map(({ label, value, color = "indigo", extra }) => (
        <Group key={label}>
          <Text size="sm">{label}:</Text>
          <Text c={color}>{value}</Text>
          {extra}
        </Group>
      ))}
    </Stack>
  );
}

export default DebuggerStatus;
