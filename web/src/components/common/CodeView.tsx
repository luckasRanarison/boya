import { instance } from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils/format";
import { ActionIcon, Group, Stack, Text } from "@mantine/core";
import { IconArrowRight, IconCircleDot } from "@tabler/icons-react";
import { useState } from "react";

type CodeLine = {
  address: number;
  value?: string;
};

function CodeView(props: {
  baseAddress: number;
  pageStart: number;
  pageSize: number;
}) {
  const { instructionCache, breakpoints } = useDebuggerStore();
  const [hovered, setHovered] = useState<number | null>(null);

  const generateLines = () => {
    const lines: CodeLine[] = [];

    for (let i = 0; i < props.pageSize; i += 2) {
      const address = props.baseAddress + props.pageStart + i;
      const instr = instructionCache[address];

      lines.push({ address, value: instr?.value });

      if (instr?.size == 4) {
        i += 2;
      }
    }

    return lines;
  };

  const pc = instance.execAddress();
  const lines = generateLines();

  return (
    <Stack w="100%" ff="monospace" gap={0}>
      {lines.map((line) => {
        const isBreakpoint = breakpoints.entries.has(line.address);

        return (
          <Group
            py="xs"
            id={`${formatHex(line.address)}`}
            style={{ scrollMarginTop: "100px" }}
            key={line.address}
            bg={
              isBreakpoint
                ? "#fa7a7a15"
                : pc === line.address
                  ? "#4c6ef515"
                  : "none"
            }
            onClick={() =>
              isBreakpoint
                ? breakpoints.remove(line.address)
                : breakpoints.add(line.address)
            }
            gap={0}
          >
            {pc === line.address ? (
              <ActionIcon
                mx="xs"
                c={isBreakpoint ? "red" : "indigo"}
                variant="transparent"
              >
                <IconArrowRight size={18} />
              </ActionIcon>
            ) : (
              <ActionIcon
                mx="xs"
                c={isBreakpoint ? "red" : "gray"}
                variant="transparent"
                opacity={hovered === line.address || isBreakpoint ? 1 : 0}
                onMouseEnter={() => setHovered(line.address)}
                onMouseLeave={() => setHovered(null)}
              >
                <IconCircleDot size={18} />
              </ActionIcon>
            )}
            <Group>
              <Text c="indigo" fw="bold">
                {formatHex(line.address)}:
              </Text>
              {line.value ? (
                <Text size="sm">{line.value}</Text>
              ) : (
                <Text c="gray" size="sm">
                  &lt;UNKNOWN&gt;
                </Text>
              )}
            </Group>
          </Group>
        );
      })}
    </Stack>
  );
}

export default CodeView;
