import { Group, Stack, Text } from "@mantine/core";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils/format";
import MemoryLink from "@/components/common/MemoryLink";
import type { CpuState } from "@/hooks/useGba";

function DebuggerStatus({ data }: { data: CpuState }) {
  const { running, cycles, lastCycle, romLoaded } = useDebuggerStore();

  const rows = [
    {
      label: "PC",
      default: formatHex(0),
      link: true,
      value: data.pc,
    },
    {
      label: "LR",
      default: formatHex(0),
      link: true,
      value: data.lr,
    },
    {
      label: "SP",
      default: formatHex(0),
      link: true,
      value: data.sp,
    },
    {
      label: "Mode",
      default: "none",
      value: data.operatingMode,
    },
    {
      label: "Cycles",
      default: 0,
      extra: lastCycle && <Text c="green">(+ {lastCycle})</Text>,
      value: cycles,
    },
  ];

  return (
    <Stack p="md">
      {rows.map((row) => (
        <Group key={row.label} justify="space-between">
          <Group>
            <Text size="sm">{row.label}:</Text>
            {romLoaded ? (
              <Text c="indigo">
                {row.link ? formatHex(row.value) : row.value}
              </Text>
            ) : (
              <Text c="gray">{row.default}</Text>
            )}
            {row.extra}
          </Group>
          {row.link && <MemoryLink address={row.value} disabled={running} />}
        </Group>
      ))}
    </Stack>
  );
}

export default DebuggerStatus;
