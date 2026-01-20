import {
  AppShell,
  Box,
  Group,
  Pagination,
  Select,
  Stack,
  Text,
  ThemeIcon,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { formatHex } from "../../utils";
import {
  IconGridDots,
  IconNoteOff,
  IconSortAscendingNumbers,
  IconStackFront,
} from "@tabler/icons-react";
import { useDebuggerStore } from "@/stores/debuggerStore";
import HexView from "./HexView";
import TileView from "./TileView";
import { memoryRegions, type MemoryRegion } from "@/lib/gba";

const viewModes = {
  hex: { pageSize: 1024 },
  tile: { pageSize: 2048 },
} as const;

function MemoryView(props: {
  region: MemoryRegion;
  columns?: number;
  mode: "hex" | "tile";
}) {
  const [currentPageId, setCurrentPageId] = useState(1);
  const { cycles } = useDebuggerStore();

  const currentEntry = memoryRegions[props.region];
  const data = currentEntry.getData();
  const { pageSize } = viewModes[props.mode];
  const columns = props.columns ?? 16;
  const pageStart = (currentPageId - 1) * pageSize;
  const total = Math.ceil(data.length / pageSize);
  const selectRegion = formatHex(currentEntry.offset + pageStart);
  const currentPage = data.slice(pageStart, pageStart + pageSize);

  const generateAddresses = () => {
    const addresses: string[] = [];

    for (let i = 0; i < total; i += 1) {
      const rawAddr = currentEntry.offset + i * pageSize;
      const hexaddr = formatHex(rawAddr);
      addresses.push(hexaddr);
    }

    return addresses;
  };

  const handleSelect = (value: string | null) => {
    if (value) {
      const basePageAddress = parseInt(value, 16) - currentEntry.offset;
      const newPage = basePageAddress / pageSize + 1;
      setCurrentPageId(newPage);
    }
  };

  const addresses = generateAddresses();

  useEffect(() => {
    // re-render component on cycle update
  }, [cycles]);

  return (
    <Stack flex={1} w="100%" align="center">
      {data.length ? (
        <>
          {props.mode === "hex" && (
            <HexView
              pageData={currentPage}
              baseAddress={currentEntry.offset}
              columns={columns}
              pageStart={pageStart}
              rightSection={props.region === "palette" ? "color" : "ascii"}
            />
          )}

          {props.mode === "tile" && <TileView pageData={currentPage} />}
        </>
      ) : (
        <Stack flex={1} justify="center" align="center" c="gray">
          <IconNoteOff size={50} />
          <Text size="xl" fw={600}>
            Empty memory region
          </Text>
        </Stack>
      )}

      <AppShell.Footer p="md">
        <Group w="100%" justify="space-between">
          <Group c="gray" visibleFrom="md">
            <ThemeIcon variant="transparent">
              <IconStackFront />
            </ThemeIcon>
            <Text ff="monospace">
              {formatHex(currentEntry.offset + pageStart)}{" "}
              {data.length ? (
                <>
                  - {formatHex(currentEntry.offset + currentPageId * pageSize)}
                </>
              ) : undefined}
            </Text>
          </Group>
          <Group w={{ base: "100%", md: "auto" }}>
            <Box c="indigo">
              {props.mode === "hex" && <IconSortAscendingNumbers />}
              {props.mode === "tile" && <IconGridDots />}
            </Box>
            <Select
              value={selectRegion}
              data={addresses}
              onChange={handleSelect}
              flex={1}
              searchable
            />
            <Pagination
              value={currentPageId}
              onChange={setCurrentPageId}
              total={total}
              withPages={false}
            />
          </Group>
        </Group>
      </AppShell.Footer>
    </Stack>
  );
}

export default MemoryView;
