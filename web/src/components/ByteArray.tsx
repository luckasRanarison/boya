import {
  Group,
  Pagination,
  ScrollArea,
  SimpleGrid,
  Stack,
  Text,
} from "@mantine/core";
import { useMemo, useState } from "react";

type ByteLine = {
  address: number;
  columns: number[];
  ascii: string;
};

function ByteArray(params: {
  data: Uint8Array;
  baseAddress: number;
  pageSize?: number;
  column?: number;
}) {
  const [currentPage, setCurrentPage] = useState(1);

  const matrix = useMemo(() => {
    const pageSize = params.pageSize ?? 1024;
    const column = params.column ?? 16;
    const start = (currentPage - 1) * pageSize;

    const slice = params.data.slice(start, start + pageSize);
    const lines: ByteLine[] = [];

    for (let i = 0; i < slice.length; i += column) {
      const row = slice.slice(i, i + column);
      const bytes = Array.from(row);

      lines.push({
        address: params.baseAddress + start + i,
        columns: bytes,
        ascii: bytes
          .map((b) => (b >= 32 && b <= 126 ? String.fromCharCode(b) : "."))
          .join(""),
      });
    }

    return lines;
  }, [
    currentPage,
    params.data,
    params.column,
    params.baseAddress,
    params.pageSize,
  ]);

  return (
    <Stack flex={1} w="100%" p="xl" justify="center" align="center">
      <ScrollArea w="100%" h="400">
        <Stack w="100%" ff={"monospace"} align="center">
          {matrix.map((line) => (
            <Group key={line.address} w="100%" justify="space-between">
              <Text c="indigo" fw={600}>
                0x{line.address.toString(16).padStart(8, "0")}:
              </Text>

              <SimpleGrid
                spacing="md"
                cols={{ base: 8, sm: 16 }}
                w={{ base: "100%", sm: "auto" }}
              >
                {line.columns.map((byte, idx) => (
                  <Text key={line.address + idx} c="gray">
                    {byte.toString(16).padStart(2, "0")}
                  </Text>
                ))}
              </SimpleGrid>

              <Text w={`${(params.column ?? 16) * 1}ch`} c="indigo.4">
                {line.ascii}
              </Text>
            </Group>
          ))}
        </Stack>
      </ScrollArea>

      <Pagination
        value={currentPage}
        onChange={setCurrentPage}
        total={params.data.length / (params.pageSize ?? 1024)}
      />
    </Stack>
  );
}

export default ByteArray;
