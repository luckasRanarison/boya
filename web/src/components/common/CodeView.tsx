import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils";
import { Group, Stack, Text } from "@mantine/core";

type CodeLine = {
  address: number;
  value?: string;
};

function CodeView(props: {
  baseAddress: number;
  pageStart: number;
  pageSize: number;
}) {
  const { instructionCache } = useDebuggerStore();

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

  const lines = generateLines();

  return (
    <Stack w="100%" p="xl" ff="monospace">
      {lines.map((line) => (
        <Group key={line.address}>
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
      ))}
    </Stack>
  );
}

export default CodeView;
