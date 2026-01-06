import { Divider, Stack } from "@mantine/core";
import RegisterBankView from "./RegisterBankView";
import DebuggerControls from "./DebuggerControls";
import DebuggerStatus from "./DebuggerStatus";
import InstructionPipeline from "./InstructionPipeline";
import { useDebuggerStore } from "../../../stores/debuggerStore";

function DebuggerView() {
  // FIXME: hack to re-render the entire component
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { cycles: _ } = useDebuggerStore();

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
