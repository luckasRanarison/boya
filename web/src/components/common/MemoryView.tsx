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
import { formatHex } from "../../utils";
import {
  IconDotsVertical,
  IconGridDots,
  IconNoteOff,
  IconSortAscendingNumbers,
  IconSourceCode,
  IconStackFront,
} from "@tabler/icons-react";
import { useDebuggerStore } from "@/stores/debuggerStore";
import HexView from "./HexView";
import TileView from "./TileView";
import { memoryRegions, type MemoryRegion } from "@/lib/gba";
import CodeView from "./CodeView";

const viewModes = [
  {
    name: "hex",
    pageSize: 1024,
    icon: IconSortAscendingNumbers,
  },
  {
    name: "tile",
    pageSize: 2048,
    icon: IconGridDots,
  },
  {
    name: "code",
    pageSize: 512,
    icon: IconSourceCode,
  },
] as const;

function MemoryView(props: { region: MemoryRegion; columns?: number }) {
  const [currentPageId, setCurrentPageId] = useState(1);
  const [currentModeId, setCurrentModeId] = useState(0);
  const { cycles } = useDebuggerStore();

  const currentEntry = memoryRegions[props.region];
  const data = currentEntry.getData();
  const {
    name: currentMode,
    pageSize,
    icon: ModeIcon,
  } = viewModes[currentModeId];
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
          {currentMode === "hex" && (
            <HexView
              pageData={currentPage}
              baseAddress={currentEntry.offset}
              columns={columns}
              pageStart={pageStart}
              rightSection={props.region === "palette" ? "color" : "ascii"}
            />
          )}

          {currentMode === "tile" && <TileView pageData={currentPage} />}

          {currentMode === "code" && (
            <CodeView
              baseAddress={currentEntry.offset}
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
                {viewModes.map(({ icon: Icon, ...mode }, id) => (
                  <Menu.Item
                    key={mode.name}
                    leftSection={<Icon size={16} />}
                    onClick={() => setCurrentModeId(id)}
                  >
                    <Text ml="xs" size="md">
                      {mode.name}
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
                {formatHex(currentEntry.offset + pageStart)}{" "}
                {data.length ? (
                  <>
                    -{" "}
                    {formatHex(currentEntry.offset + currentPageId * pageSize)}
                  </>
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
