import { Group, Stack, Text } from "@mantine/core";
import { useRuntimeStore } from "@/stores/runtimeStore";
import { formatHex } from "@/utils/format";
import MemoryLink from "@/components/common/MemoryLink";
import type { GbaState } from "@/hooks/useGba";

function DebuggerStatus(props: { state: GbaState; running: boolean }) {
  const lastCycle = useRuntimeStore((state) => state.lastCycle);

  const rows = [
    {
      label: "PC",
      default: formatHex(0),
      link: true,
      value: props.state.cpu.pc,
    },
    {
      label: "LR",
      default: formatHex(0),
      link: true,
      value: props.state.cpu.lr,
    },
    {
      label: "SP",
      default: formatHex(0),
      link: true,
      value: props.state.cpu.sp,
    },
    {
      label: "Mode",
      default: "none",
      value: props.state.cpu.operatingMode,
    },
    {
      label: "Scanline",
      default: "0",
      value: props.state.scanline,
    },
    {
      label: "Cycles",
      default: 0,
      extra: lastCycle && <Text c="green">(+ {lastCycle})</Text>,
      value: props.state.cycles.toString(),
    },
  ];

  return (
    <Stack p="md">
      {rows.map((row) => (
        <Group key={row.label} justify="space-between">
          <Group>
            <Text size="sm">{row.label}:</Text>
            {props.state.booted ? (
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
