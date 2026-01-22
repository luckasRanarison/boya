import {
  ActionIcon,
  AppShell,
  Group,
  Menu,
  Pagination,
  Select,
  Stack,
  Text,
  ThemeIcon,
  Tooltip,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { formatHex } from "@/utils/format";
import {
  IconDotsVertical,
  IconGridDots,
  IconNoteOff,
  IconSortAscendingNumbers,
  IconSourceCode,
  IconStackFront,
} from "@tabler/icons-react";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { memoryRegions, type MemoryRegion } from "@/lib/gba";
import { useMemoryPage } from "@/hooks/useMemoryPage";
import { usePersistantStore } from "@/stores/persistantStore";

import HexView from "./HexView";
import TileView from "./TileView";
import CodeView from "./CodeView";

const viewModes = {
  hex: {
    pageSize: 1024,
    icon: IconSortAscendingNumbers,
  },
  tile: {
    pageSize: 2048,
    icon: IconGridDots,
  },
  code: {
    pageSize: 512,
    icon: IconSourceCode,
  },
};

export type MemoryViewMode = keyof typeof viewModes;

export type MemoryViewProps = {
  region: MemoryRegion;
  mode: MemoryViewMode;
  targetAddress?: number;
  columns?: number;
};

function MemoryView(props: MemoryViewProps) {
  const { cycles, decode } = useDebuggerStore();
  const { decodeDepth } = usePersistantStore();

  const [currentMode, setCurrentMode] = useState(props.mode ?? "hex");
  const { offset, getData } = memoryRegions[props.region];
  const { pageSize, icon: ModeIcon } = viewModes[currentMode];

  const [{ pageId }, dispatch] = useMemoryPage({ offset, pageSize });

  const data = getData();
  const columns = props.columns ?? 16;
  const pageStart = (pageId - 1) * pageSize;
  const total = Math.ceil(data.length / pageSize);
  const selectRegion = formatHex(offset + pageStart);
  const currentPage = data.slice(pageStart, pageStart + pageSize);

  const generateAddresses = () => {
    const addresses: string[] = [];

    for (let i = 0; i < total; i += 1) {
      const rawAddr = offset + i * pageSize;
      const hexaddr = formatHex(rawAddr);
      addresses.push(hexaddr);
    }

    return addresses;
  };

  const handleSelect = (value: string | null) => {
    if (value) {
      const basePageAddress = parseInt(value, 16) - offset;
      const newPage = basePageAddress / pageSize + 1;
      dispatch({ type: "select", pageId: newPage });
    }
  };

  const addresses = generateAddresses();

  useEffect(() => {
    if (props.targetAddress) {
      dispatch({ type: "jump", address: props.targetAddress });
    }
  }, [props.targetAddress, dispatch]);

  useEffect(() => {
    if (currentMode === "code") {
      decode(decodeDepth);
    }
  }, [cycles, currentMode, decodeDepth, decode]);

  return (
    <Stack flex={1} mb="80px" align="center">
      {data.length ? (
        <>
          {currentMode === "hex" && (
            <HexView
              pageData={currentPage}
              baseAddress={offset}
              columns={columns}
              pageStart={pageStart}
              rightSection={props.region === "palette" ? "color" : "ascii"}
            />
          )}

          {currentMode === "tile" && <TileView pageData={currentPage} />}

          {currentMode === "code" && (
            <CodeView
              baseAddress={offset}
              pageStart={pageStart}
              pageSize={pageSize}
            />
          )}
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
          <Group>
            <Menu width="150" position="top-start" offset={35}>
              <Menu.Target>
                <ActionIcon variant="subtle" c="gray">
                  <IconDotsVertical />
                </ActionIcon>
              </Menu.Target>
              <Menu.Dropdown>
                {Object.entries(viewModes).map(([name, { icon: Icon }]) => (
                  <Menu.Item
                    key={name}
                    leftSection={<Icon size={16} />}
                    onClick={() => setCurrentMode(name as MemoryViewMode)}
                  >
                    <Text ml="xs" size="md">
                      {name}
                    </Text>
                  </Menu.Item>
                ))}
              </Menu.Dropdown>
            </Menu>
            <Group c="gray" visibleFrom="md">
              <ThemeIcon variant="transparent">
                <IconStackFront />
              </ThemeIcon>
              <Text ff="monospace">
                {formatHex(offset + pageStart)}{" "}
                {data.length ? (
                  <>- {formatHex(offset + pageId * pageSize)}</>
                ) : undefined}
              </Text>
            </Group>
          </Group>

          <Group flex={{ base: 1, sm: "inherit" }}>
            <Tooltip label={`${currentMode} mode`}>
              <ThemeIcon size="input-sm" variant="transparent">
                <ModeIcon />
              </ThemeIcon>
            </Tooltip>
            <Select
              value={selectRegion}
              data={addresses}
              onChange={handleSelect}
              flex={1}
              searchable
            />
            <Pagination
              value={pageId}
              onChange={(pageId) => dispatch({ type: "select", pageId })}
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
