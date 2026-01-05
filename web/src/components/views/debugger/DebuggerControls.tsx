import { ActionIcon, Group, Tooltip } from "@mantine/core";
import {
  IconArrowBack,
  IconArrowBackUp,
  IconClockPlay,
  IconPlayerPlay,
  IconRestore,
  IconStepInto,
  IconStepOut,
} from "@tabler/icons-react";
import { useMemo } from "react";
import { useDebuggerStore } from "../../../stores/debuggerStore";

function DebuggerControls() {
  const { stepInto } = useDebuggerStore();

  const actions = useMemo(
    () => [
      {
        icon: IconRestore,
        label: "Reset",
        onClick: () => console.log("reset"),
      },
      {
        icon: IconArrowBackUp,
        label: "Undo",
        onClick: () => console.log("undo"),
      },
      {
        icon: IconArrowBack,
        label: "Step back",
        onClick: () => console.log("step back"),
      },
      {
        icon: IconPlayerPlay,
        label: "Continue",
        onClick: () => console.log("continue"),
      },
      {
        icon: IconStepInto,
        label: "Step into",
        onClick: () => stepInto(),
      },
      {
        icon: IconStepOut,
        label: "Step out",
        onClick: () => console.log("step out"),
      },
      {
        icon: IconClockPlay,
        label: "Run",
        onClick: () => console.log("run"),
      },
    ],
    [stepInto],
  );

  return (
    <Group mb="md" w="100%" justify="center">
      {actions.map(({ icon: Icon, label, onClick }, i) => (
        <Tooltip key={i} label={label}>
          <ActionIcon variant="subtle" onClick={onClick}>
            <Icon />
          </ActionIcon>
        </Tooltip>
      ))}
    </Group>
  );
}

export default DebuggerControls;
