import { Accordion, ActionIcon, Divider, Stack, Tooltip } from "@mantine/core";
import CPURegisterView from "../registers/CPURegisterView";
import DebuggerControls from "./DebuggerControls";
import DebuggerStatus from "./DebuggerStatus";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { useState } from "react";
import IORegisterView from "../registers/IORegisterView";
import BreakpointControl from "./BreakpointControl";
import FloatingControl from "./FloatingControl";
import { IconFoldDown } from "@tabler/icons-react";
import { useGba } from "@/hooks/useGba";
import PipelineView from "./PipelineView";

function DebuggerView() {
  const { cpu, memory } = useGba();
  const { running, cycles, panel } = useDebuggerStore();

  const [activeMenu, setActiveMenu] = useState<string[]>([
    "status",
    "pipeline",
  ]);

  const menus = [
    {
      key: "breakpoint",
      label: "Breakpoints",
      view: <BreakpointControl />,
    },
    {
      key: "status",
      label: "Status",
      view: <DebuggerStatus data={cpu} />,
    },
    {
      key: "pipeline",
      label: "Pipeline",
      view: <PipelineView base={cpu.pc} pipeline={cpu.pipeline()} />,
    },
    {
      key: "cpu_reg",
      label: "CPU Registers",
      view: <CPURegisterView value={cpu.getRegisters()} style="simple" />,
    },
    {
      key: "io_reg",
      label: "I/O Registers",
      view: <IORegisterView value={memory.getIoRegisters()} style="simple" />,
    },
  ];

  return (
    <Stack
      w="100%"
      pb="20dvh"
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
