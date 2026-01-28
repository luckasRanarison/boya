import { GBA } from "@/lib/gba";
import { useEffect, useState } from "react";
import { useGba } from "@/hooks/useGba";
import { useRuntimeStore } from "@/stores/runtimeStore";
import { useViewActions, useViewStore } from "@/stores/viewStore";
import { Accordion, ActionIcon, Divider, Stack, Tooltip } from "@mantine/core";
import { useDebuggerActions } from "@/stores/debuggerStore";
import CPURegisterView from "../registers/CPURegisterView";
import DebuggerControls from "./DebuggerControls";
import DebuggerStatus from "./DebuggerStatus";
import IORegisterView from "../registers/IORegisterView";
import BreakpointControl from "./BreakpointControl";
import FloatingControl from "./FloatingControl";
import { IconFoldDown } from "@tabler/icons-react";
import PipelineView from "./PipelineView";
import CallStack from "./CallStack";

function DebuggerView() {
  const { cpu, cycles, memory, booted } = useGba();
  const running = useRuntimeStore((state) => state.running);
  const debugPannel = useViewStore((state) => state.debugPannel);
  const view = useViewStore((state) => state.view);
  const { toggleDebugPannel, moveDebugPannel } = useViewActions();
  const { decode, pushStack, popStack } = useDebuggerActions();

  const [activeMenu, setActiveMenu] = useState([
    "status",
    "pipeline",
    "callstack",
  ]);

  useEffect(() => {
    const pc = GBA.execAddress();

    if ((cpu.lr & ~1) === pc) {
      popStack();
    }

    if (GBA.startingSubroutine()) {
      pushStack({ caller: pc, return: pc + GBA.instructionSize() });
    }

    if (!(view.name === "memory" && view.sub?.metadata?.mode === "code")) {
      decode(2);
    }
  }, [cpu.lr, cycles, view, decode, pushStack, popStack]);

  const menus = [
    {
      key: "breakpoint",
      label: "Breakpoints",
      view: <BreakpointControl disabled={running} />,
    },
    {
      key: "status",
      label: "Status",
      view: (
        <DebuggerStatus
          cpu={cpu}
          running={running}
          booted={booted}
          cycles={cycles}
        />
      ),
    },
    {
      key: "pipeline",
      label: "Pipeline",
      view: <PipelineView base={cpu.pc} pipeline={cpu.pipeline()} />,
    },
    {
      key: "callstack",
      label: "Call Stack",
      view: <CallStack disabled={running} />,
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
        {debugPannel.floating ? (
          <>
            <FloatingControl
              defaultValue={debugPannel.position}
              onChange={moveDebugPannel}
            />
            <Tooltip label="Dock">
              <ActionIcon variant="subtle" onClick={toggleDebugPannel}>
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
