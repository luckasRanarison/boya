import {
  ActionIcon,
  AppShell,
  Group,
  Input,
  Menu,
  Pagination,
  Stack,
  Text,
  ThemeIcon,
  Tooltip,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { formatHex, parseHex } from "@/utils/format";
import {
  IconDotsVertical,
  IconGridDots,
  IconNoteOff,
  IconSortAscendingNumbers,
  IconSourceCode,
  IconStackFront,
} from "@tabler/icons-react";
import { useMemoryPage } from "@/hooks/useMemoryPage";
import { useGba } from "@/hooks/useGba";

import HexView from "./HexView";
import TileView from "./TileView";
import CodeView from "./CodeView";
import type { MemoryRegionName } from "@/lib/gba";
import { useGotoMemory } from "@/hooks/useGotoMemory";
import { useSearchParams } from "react-router";
import { useActiveRoute } from "@/hooks/useActiveRoute";

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
  region: MemoryRegionName;
  mode: MemoryViewMode;
  jump?: { address: number };
};

function MemoryView() {
  const { activeRoute } = useActiveRoute();
  const [searchParams] = useSearchParams();
  const mode = searchParams.get("mode") as MemoryViewMode | undefined;
  const jump = searchParams.get("jump");
  const regionName = activeRoute?.path as MemoryRegionName | undefined;

  const { memory, cpu } = useGba();
  const [currentMode, setCurrentMode] = useState(mode ?? "hex");

  const { offset, ...region } = memory.getRegion(regionName ?? "bios");
  const { pageSize, icon: ModeIcon } = viewModes[currentMode];

  const [{ pageId }, dispatch] = useMemoryPage({ offset, pageSize });

  const pageStart = (pageId - 1) * pageSize;
  const total = Math.ceil(region.getLength() / pageSize);
  const currentPage = region.getData(pageStart, pageStart + pageSize);
  const renderkey = offset + pageStart;

  const gotoMemory = useGotoMemory();

  const handleGoto = (address: number) => {
    gotoMemory({
      mode: currentMode,
      hightlight: true,
      address,
    });
  };

  useEffect(() => {
    if (jump) {
      const [address] = jump.split(".");
      const parsed = parseInt(address);

      dispatch({ type: "jump", address: parsed });
    }
  }, [jump, dispatch]);

  return (
    <Stack flex={1} mb="80px" align="center">
      {currentPage?.length ? (
        <>
          {currentMode === "hex" && (
            <HexView
              key={renderkey}
              pageData={currentPage}
              baseAddress={offset}
              columns={16}
              pageStart={pageStart}
              rightSection={regionName === "palette" ? "color" : "ascii"}
            />
          )}

          {currentMode === "tile" && (
            <TileView
              key={renderkey}
              pageStart={pageStart}
              pageData={currentPage}
            />
          )}

          {currentMode === "code" && (
            <CodeView
              execAddress={cpu.pc}
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
                {currentPage?.length ? (
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
            <AddressInput onConfirm={handleGoto} />
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

function AddressInput(props: { onConfirm: (value: number) => void }) {
  const [value, setValue] = useState("");

  const handleConfirm: React.FormEventHandler = (event) => {
    const parsed = parseHex(value);

    if (!Number.isNaN(parsed)) {
      props.onConfirm(parsed);
    }

    event.preventDefault();
  };

  return (
    <form style={{ flex: 1 }} onSubmit={handleConfirm}>
      <Input
        placeholder="Go to address..."
        onChange={(e) => setValue(e.currentTarget.value)}
        error={value && Number.isNaN(parseHex(value))}
        value={value}
      />
    </form>
  );
}

export default MemoryView;
