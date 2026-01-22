import { Accordion, ActionIcon, Divider, Stack, Tooltip } from "@mantine/core";
import CPURegisterView from "../cpu/CPURegisterView";
import DebuggerControls from "./DebuggerControls";
import DebuggerStatus from "./DebuggerStatus";
import InstructionPipeline from "./InstructionPipeline";
import { useDebuggerStore } from "../../../stores/debuggerStore";
import { useEffect, useState } from "react";
import IORegisterView from "../io/IORegisterView";
import BreakpointControl from "./BreakpointControl";
import FloatingControl from "./FloatingControl";
import { useView } from "@/stores/viewStore";
import { IconFoldDown } from "@tabler/icons-react";

const menus = [
  {
    key: "breakpoint",
    label: "Breakpoints",
    view: <BreakpointControl />,
  },
  {
    key: "status",
    label: "Status",
    view: <DebuggerStatus />,
  },
  {
    key: "pipeline",
    label: "Pipeline",
    view: <InstructionPipeline />,
  },
  {
    key: "cpu_reg",
    label: "CPU Registers",
    view: <CPURegisterView style="simple" />,
  },
  {
    key: "io_reg",
    label: "I/O Registers",
    view: <IORegisterView style="simple" />,
  },
];

function DebuggerView() {
  const { running, cycles, panel, decode } = useDebuggerStore();
  const { view } = useView();

  const [activeMenu, setActiveMenu] = useState<string[]>([
    "status",
    "pipeline",
  ]);

  const isCodeView =
    view.name === "memory" && view.sub?.metadata?.mode === "code";

  // re-render the component and decode next instructions on cycle update
  useEffect(() => {
    if (!isCodeView) {
      decode(2);
    }
  }, [decode, cycles, isCodeView]);

  return (
    <Stack
      w="100%"
      pb="10dvh"
      mah="90dvh"
      style={{ overflow: "scroll" }}
      ff="monospace"
    >
      <Stack pt="xl" pb="md" align="center">
        {panel.floating ? (
          <>
            <FloatingControl
              defaultValue={panel.position}
              onChange={panel.setPosition}
            />
            <Tooltip label="Dock">
              <ActionIcon variant="subtle" onClick={panel.toggleFloat}>
                <IconFoldDown />
              </ActionIcon>
            </Tooltip>
          </>
        ) : (
          <DebuggerControls />
        )}
      </Stack>

      <Accordion multiple value={activeMenu} onChange={setActiveMenu}>
        <Divider />

        {menus.map(({ key, label, view }) => (
          <Accordion.Item key={`${key}-${cycles}`} value={key}>
            <Accordion.Control fz="sm" disabled={running}>
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
