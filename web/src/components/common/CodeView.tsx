import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils";
import { Group, Stack, Text } from "@mantine/core";
import { useEffect } from "react";

function CodeView(props: {
  baseAddress: number;
  pageStart: number;
  pageSize: number;
}) {
  const { instructionCache, decode } = useDebuggerStore();

  useEffect(() => {
    decode();
  }, []);

  return (
    <Stack w="100%" p="xl" ff="monospace">
      {new Array(props.pageSize / 2).fill(0).map((_, i) => {
        const address = props.baseAddress + props.pageStart + i * 2;

        if (instructionCache[address]) {
          return (
            <Group key={i}>
              <Text c="indigo" fw="bold">
                {formatHex(address)}:
              </Text>
              <Text>{instructionCache[address]}</Text>
            </Group>
          );
        }
      })}
    </Stack>
  );
}

export default CodeView;
