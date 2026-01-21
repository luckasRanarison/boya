import { Divider, Stack } from "@mantine/core";
import RegisterBankView from "./RegisterBankView";
import DebuggerControls from "./DebuggerControls";
import DebuggerStatus from "./DebuggerStatus";
import InstructionPipeline from "./InstructionPipeline";
import { useDebuggerStore } from "../../../stores/debuggerStore";
import { useEffect } from "react";

function DebuggerView() {
  const { cycles, decode } = useDebuggerStore();

  // re-render the component on cycle update
  useEffect(() => {
    decode(10);
  }, [cycles]);

  return (
    <Stack
      w="100%"
      pt="xl"
      pb="20dvh"
      px="md"
      mah="90dvh"
      style={{ overflow: "scroll" }}
      ff="monospace"
    >
      <DebuggerControls />
      <DebuggerStatus />
      <InstructionPipeline />
      <Divider />
      <RegisterBankView />
    </Stack>
  );
}

export default DebuggerView;
