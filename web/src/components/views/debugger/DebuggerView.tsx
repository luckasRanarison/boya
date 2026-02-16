import { useState } from "react";
import { useGba } from "@/hooks/useGba";
import { useRuntimeStore } from "@/stores/runtimeStore";
import { useViewActions, useViewStore } from "@/stores/viewStore";
import { Accordion, ActionIcon, Divider, Stack, Tooltip } from "@mantine/core";
import DebuggerControls from "./DebuggerControls";
import FloatingControl from "./FloatingControl";
import { IconFoldDown, IconFoldUp } from "@tabler/icons-react";
import { useDebuggerMenu } from "@/hooks/useDebuggerMenu";

function DebuggerView() {
  const [activeMenu, setActiveMenu] = useState(["status", "pipeline"]);

  const state = useGba();
  const menus = useDebuggerMenu(state);
  const running = useRuntimeStore((state) => state.running);
  const panelPosition = useViewStore((state) => state.debugPanelPosition);
  const floatingWindows = useViewStore((state) => state.floatingWindows);

  const { toggleWindow, moveDebugPanel } = useViewActions();

  return (
    <Stack
      w="100%"
      pb="20dvh"
      mah="90dvh"
      style={{ overflow: "scroll" }}
      ff="monospace"
    >
      <Stack pt="xl" pb="md" align="center">
        {floatingWindows.includes("panel") ? (
          <>
            <FloatingControl
              defaultValue={panelPosition}
              onChange={moveDebugPanel}
            />
            <Tooltip label="Dock">
              <ActionIcon
                variant="subtle"
                onClick={() => toggleWindow("panel")}
              >
                <IconFoldDown />
              </ActionIcon>
            </Tooltip>
          </>
        ) : (
          <DebuggerControls />
        )}
      </Stack>

      <Accordion
        value={activeMenu}
        onChange={setActiveMenu}
        chevronPosition="left"
        multiple
      >
        <Divider />

        {menus
          .filter((m) => !floatingWindows.includes(m.key))
          .map(({ key, label, view }) => (
            <Accordion.Item key={`${key}-${state.cycles}`} value={key}>
              <Accordion.Control
                icon={
                  <ActionIcon
                    title="Detach"
                    size="md"
                    variant="subtle"
                    onClick={() => toggleWindow(key)}
                    component="div"
                  >
                    <IconFoldUp />
                  </ActionIcon>
                }
                fz="sm"
                disabled={running}
              >
                {label}
              </Accordion.Control>
              <Accordion.Panel
                styles={{
                  content: {
                    padding: 0,
                  },
                }}
              >
                {activeMenu.includes(key) && view}
              </Accordion.Panel>
            </Accordion.Item>
          ))}
      </Accordion>
    </Stack>
  );
}

export default DebuggerView;
