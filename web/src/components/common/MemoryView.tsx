import {
  ActionIcon,
  AppShell,
  Group,
  Pagination,
  Select,
  Stack,
  Text,
  ThemeIcon,
  Tooltip,
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

const viewModes = [
  { name: "hex", pageSize: 1024 },
  { name: "tile", pageSize: 2048 },
] as const;

function MemoryView(props: {
  data: Uint8Array;
  baseAddress: number;
  columns?: number;
  rightSection?: "ascii" | "color";
}) {
  const [currentPageId, setCurrentPageId] = useState(1);
  const [currentModeId, setCurrentModeId] = useState(0);
  const { cycles } = useDebuggerStore();

  const { name: currentMode, pageSize } = viewModes[currentModeId];
  const columns = props.columns ?? 16;
  const pageStart = (currentPageId - 1) * pageSize;
  const total = Math.ceil(props.data.length / pageSize);
  const selectRegion = formatHex(props.baseAddress + pageStart);
  const currentPage = props.data.slice(pageStart, pageStart + pageSize);

  const generateAddresses = () => {
    const addresses: string[] = [];

    for (let i = 0; i < total; i += 1) {
      const rawAddr = props.baseAddress + i * pageSize;
      const hexaddr = formatHex(rawAddr);
      addresses.push(hexaddr);
    }

    return addresses;
  };

  const handleSelect = (value: string | null) => {
    if (value) {
      const basePageAddress = parseInt(value, 16) - props.baseAddress;
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
      {props.data.length ? (
        <>
          {currentMode === "hex" && (
            <HexView
              pageData={currentPage}
              baseAddress={props.baseAddress}
              columns={columns}
              pageStart={pageStart}
              rightSection={props.rightSection}
            />
          )}

          {currentMode === "tile" && <TileView pageData={currentPage} />}
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
              {formatHex(props.baseAddress + pageStart)}{" "}
              {props.data.length ? (
                <>- {formatHex(props.baseAddress + currentPageId * pageSize)}</>
              ) : undefined}
            </Text>
          </Group>
          <Group w={{ base: "100%", md: "auto" }}>
            <Tooltip label={`Toggle ${currentMode} mode`}>
              <ActionIcon
                size="input-sm"
                variant="outline"
                onClick={() =>
                  setCurrentModeId((prev) => (prev + 1) % viewModes.length)
                }
              >
                {currentMode === "hex" && <IconSortAscendingNumbers />}
                {currentMode === "tile" && <IconGridDots />}
              </ActionIcon>
            </Tooltip>
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
