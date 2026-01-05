import { Divider, Stack } from "@mantine/core";
import RegisterBankView from "./RegisterBankView";
import DebuggerControls from "./DebuggerControls";
import DebuggerStatus from "./DebuggerStatus";
import InstructionPipeline from "./InstructionPipeline";

function DebuggerView() {
  return (
    <Stack
      w="100%"
      pt="xl"
      pb="10dvh"
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
