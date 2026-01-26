import { Group, Stack, Text } from "@mantine/core";
import { useRuntimeStore } from "@/stores/runtimeStore";
import { formatHex } from "@/utils/format";
import MemoryLink from "@/components/common/MemoryLink";
import type { CpuState } from "@/hooks/useGba";

function DebuggerStatus(props: {
  cpu: CpuState;
  running: boolean;
  booted: boolean;
  cycles: bigint;
}) {
  const lastCycle = useRuntimeStore((state) => state.lastCycle);

  const rows = [
    {
      label: "PC",
      default: formatHex(0),
      link: true,
      value: props.cpu.pc,
    },
    {
      label: "LR",
      default: formatHex(0),
      link: true,
      value: props.cpu.lr,
    },
    {
      label: "SP",
      default: formatHex(0),
      link: true,
      value: props.cpu.sp,
    },
    {
      label: "Mode",
      default: "none",
      value: props.cpu.operatingMode,
    },
    {
      label: "Cycles",
      default: 0,
      extra: lastCycle && <Text c="green">(+ {lastCycle})</Text>,
      value: props.cycles.toString(),
    },
  ];

  return (
    <Stack p="md">
      {rows.map((row) => (
        <Group key={row.label} justify="space-between">
          <Group>
            <Text size="sm">{row.label}:</Text>
            {props.booted ? (
              <Text c="indigo">
                {row.link ? formatHex(row.value) : row.value}
              </Text>
            ) : (
              <Text c="gray">{row.default}</Text>
            )}
            {row.extra}
          </Group>
          {row.link && (
            <MemoryLink address={row.value} disabled={props.running} />
          )}
        </Group>
      ))}
    </Stack>
  );
}

export default DebuggerStatus;
