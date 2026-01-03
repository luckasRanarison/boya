import { Group, ScrollArea, Stack, Text } from "@mantine/core";
import { useMemo } from "react";

type ByteLine = {
  address: number;
  columns: number[];
};

const convertToMatrix = (
  array: Uint8Array,
  baseAddress: number,
  column = 16,
) => {
  const lines: ByteLine[] = [];

  for (let i = 0; i < array.length; i += column) {
    const slice = array.slice(i, i + column);

    lines.push({
      address: baseAddress + i,
      columns: Array.from(slice),
    });
  }

  return lines;
};

function ByteArray(params: {
  data: Uint8Array;
  baseAddress: number;
  column?: number;
}) {
  const matrix = useMemo(
    () => convertToMatrix(params.data, params.baseAddress, params.column),
    [params.data, params.column],
  );

  return (
    <ScrollArea w="100%" h="500" p="md">
      <Stack ff={"monospace"}>
        {matrix.map((line) => (
          <Group key={line.address}>
            <Text mr="xl" c="indigo" fw={600}>
              0x{line.address.toString(16).padStart(8, "0")}
            </Text>
            <Group>
              {line.columns.map((byte, idx) => (
                <Text key={line.address + idx} w="30" c="gray">
                  {byte.toString(16).padStart(2, "0")}
                </Text>
              ))}
            </Group>
          </Group>
        ))}
      </Stack>
    </ScrollArea>
  );
}

export default ByteArray;
