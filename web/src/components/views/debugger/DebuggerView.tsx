import { Accordion, Divider, Group, Stack } from "@mantine/core";
import CPURegisterView from "../cpu/CPURegisterView";
import DebuggerControls from "./DebuggerControls";
import DebuggerStatus from "./DebuggerStatus";
import InstructionPipeline from "./InstructionPipeline";
import { useDebuggerStore } from "../../../stores/debuggerStore";
import { useEffect, useState } from "react";
import IORegisterView from "../io/IORegisterView";
import BreakpointControl from "./BreakpointControl";
import FloatingControl from "./FloatingControl";

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
  const { cycles, panel, decode } = useDebuggerStore();

  const [activeMenu, setActiveMenu] = useState<string[]>([
    "status",
    "pipeline",
  ]);

  // re-render the component and decode next instructions on cycle update
  useEffect(() => {
    decode(10);
  }, [decode, cycles]);

  return (
    <Stack
      w="100%"
      pb="10dvh"
      mah="90dvh"
      style={{ overflow: "scroll" }}
      ff="monospace"
    >
      <Group pt="xl" pb="md" justify="center">
        {panel.floating ? (
          <FloatingControl
            defaultValue={panel.position}
            onChange={panel.setPosition}
          />
        ) : (
          <DebuggerControls />
        )}
      </Group>

      <Accordion multiple value={activeMenu} onChange={setActiveMenu}>
        <Divider />

        {menus.map(({ key, label, view }) => (
          <Accordion.Item key={`${key}-${cycles}`} value={key}>
            <Accordion.Control fz="sm">{label}</Accordion.Control>
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
