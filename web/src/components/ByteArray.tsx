import {
  AppShell,
  Group,
  Pagination,
  Select,
  SimpleGrid,
  Stack,
  Text,
  ThemeIcon,
} from "@mantine/core";
import { useMemo, useState } from "react";
import { formatHex } from "../utils";
import { IconSortAscendingNumbers, IconStackFront } from "@tabler/icons-react";

type ByteLine = {
  address: number;
  columns: number[];
  ascii: string;
};

function ByteArray(params: {
  data: Uint8Array;
  baseAddress: number;
  pageSize?: number;
  columns?: number;
}) {
  const [currentPage, setCurrentPage] = useState(1);

  const pageSize = params.pageSize ?? 1024;
  const columns = params.columns ?? 16;
  const start = (currentPage - 1) * pageSize;
  const total = Math.ceil(params.data.length / (params.pageSize ?? 1024));
  const selectRegion = formatHex(params.baseAddress + start);

  const { lines, addresses } = useMemo(() => {
    const slice = params.data.slice(start, start + pageSize);
    const lines: ByteLine[] = [];
    const addresses: string[] = [];

    for (let i = 0; i < slice.length; i += columns) {
      const row = slice.slice(i, i + columns);
      const bytes = Array.from(row);

      lines.push({
        address: params.baseAddress + start + i,
        columns: bytes,
        ascii: bytes
          .map((b) => (b >= 32 && b <= 126 ? String.fromCharCode(b) : "."))
          .join(""),
      });
    }

    for (let i = 0; i < total; i += 1) {
      addresses.push(formatHex(params.baseAddress + i * pageSize));
    }

    return { lines, addresses };
  }, [start, columns, pageSize, total, params.data, params.baseAddress]);

  const handleSelect = (value: string | null) => {
    if (value) {
      const basePageAddress = parseInt(value, 16) - params.baseAddress;
      const newPage = basePageAddress / pageSize + 1;
      setCurrentPage(newPage);
    }
  };

  return (
    <Stack flex={1} w="100%" p="xl" justify="space-around" align="center">
      <Stack w="100%" ff={"monospace"} align="center">
        {lines.map((line) => (
          <Group key={line.address} w="100%" justify="space-between">
            <Text c="indigo" fw={600}>
              {formatHex(line.address)}:
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

            <Text w={`${columns}ch`} c="indigo.4">
              {line.ascii}
            </Text>
          </Group>
        ))}
      </Stack>

      <AppShell.Footer p="md">
        <Group w="100%" justify="space-between">
          <Group c="gray" visibleFrom="md">
            <ThemeIcon variant="transparent">
              <IconStackFront />
            </ThemeIcon>
            <Text ff="monospace">
              {formatHex(params.baseAddress + start)} -{" "}
              {formatHex(params.baseAddress + currentPage * pageSize)}
            </Text>
          </Group>
          <Group w={{ base: "100%", md: "auto" }}>
            <ThemeIcon variant="transparent">
              <IconSortAscendingNumbers />
            </ThemeIcon>
            <Select
              value={selectRegion}
              data={addresses}
              onChange={handleSelect}
              flex={1}
              searchable
            />
            <Pagination
              value={currentPage}
              onChange={setCurrentPage}
              total={total}
              withPages={false}
            />
          </Group>
        </Group>
      </AppShell.Footer>
    </Stack>
  );
}

export default ByteArray;
