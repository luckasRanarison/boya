import { Accordion, Divider, Stack } from "@mantine/core";
import CPURegisterView from "./CPURegisterView";
import DebuggerControls from "./DebuggerControls";
import DebuggerStatus from "./DebuggerStatus";
import InstructionPipeline from "./InstructionPipeline";
import { useDebuggerStore } from "../../../stores/debuggerStore";
import { useEffect, useState } from "react";
import IORegisterView from "../io/IORegisterView";
import BreakpointControl from "./BreakpointControl";

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
  const { cycles, decode } = useDebuggerStore();

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
      pt="xl"
      pb="20dvh"
      mah="90dvh"
      style={{ overflow: "scroll" }}
      ff="monospace"
    >
      <DebuggerControls />

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
