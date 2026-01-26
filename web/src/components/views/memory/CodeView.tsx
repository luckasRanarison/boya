import { usePersistantStore } from "@/stores/persistantStore";
import styles from "./CodeView.module.css";

import { useDebuggerActions, useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils/format";
import { ActionIcon, Group, Stack, Text } from "@mantine/core";
import { IconArrowRight, IconCircleDot } from "@tabler/icons-react";
import { useEffect, useState } from "react";

type CodeLine = {
  address: number;
  value?: string;
};

function CodeView(props: {
  execAddress: number;
  baseAddress: number;
  pageStart: number;
  pageSize: number;
}) {
  const [hovered, setHovered] = useState<number | null>(null);
  const breakpoints = useDebuggerStore((state) => state.breakpoints);
  const instructionCache = useDebuggerStore((state) => state.instructionCache);
  const decodeDepth = usePersistantStore((state) => state.decodeDepth);
  const { removeBreak, addBreak, decode } = useDebuggerActions();

  useEffect(() => {
    decode(decodeDepth);
  }, [props.execAddress, decodeDepth, decode]);

  const generateLines = () => {
    const lines: CodeLine[] = [];

    for (let i = 0; i < props.pageSize; i += 2) {
      const address = props.baseAddress + props.pageStart + i;
      const instr = instructionCache[address];

      lines.push({ address, value: instr?.value });

      if (instr?.size == 4) i += 2;
    }

    return lines;
  };

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
            className={
              isBreakpoint
                ? styles["breakpoint-highlight"]
                : line.address === props.execAddress
                  ? styles["execution-highlight"]
                  : undefined
            }
            onClick={() =>
              isBreakpoint ? removeBreak(line.address) : addBreak(line.address)
            }
            gap={0}
          >
            {props.execAddress === line.address ? (
              <ActionIcon
                mx="xs"
                c={isBreakpoint ? "red" : "green"}
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
