import { ActionIcon, Group, Stack, Tooltip } from "@mantine/core";
import {
  IconArrowBack,
  IconArrowBackUp,
  IconClockPlay,
  IconPlayerPause,
  IconPlayerPlay,
  IconRestore,
  IconStepInto,
  IconStepOut,
} from "@tabler/icons-react";
import { useDebuggerStore } from "../../../stores/debuggerStore";
import { useView } from "@/stores/viewStore";
import { instance } from "@/lib/gba";

function DebuggerControls() {
  const dbg = useDebuggerStore();
  const { view, gotoMemory } = useView();

  const handleStepInto = () => {
    dbg.stepInto();

    if (view.name === "memory") {
      gotoMemory(instance.execAddress(), "code");
    }
  };

  const actions = [
    {
      icon: IconRestore,
      label: "Reset",
      onClick: () => dbg.reset(),
      disabled: dbg.running || !dbg.romLoaded,
    },
    {
      icon: IconArrowBackUp,
      label: "Undo",
      onClick: () => console.log("undo"),
      disabled: true,
    },
    {
      icon: IconArrowBack,
      label: "Step back",
      onClick: () => console.log("step back"),
      disabled: true,
    },
    {
      icon: dbg.running ? IconPlayerPause : IconPlayerPlay,
      label: dbg.running ? "Pause" : "Continue",
      onClick: () => (dbg.running ? dbg.pause() : dbg.run()),
      disabled: !dbg.romLoaded,
    },
    {
      icon: IconStepInto,
      label: "Step into",
      onClick: handleStepInto,
      disabled: dbg.running || !dbg.romLoaded,
    },
    {
      icon: IconStepOut,
      label: "Step out",
      onClick: () => console.log("step out"),
      disabled: true,
    },
    {
      icon: IconClockPlay,
      label: "Run",
      onClick: () => console.log("run"),
      disabled: true,
    },
  ];

  return (
    <Stack mb="md" px="md">
      <Group w="100%" justify="center">
        {actions.map(({ icon: Icon, label, disabled, onClick }) => (
          <Tooltip key={label} label={label}>
            <ActionIcon
              variant="subtle"
              onClick={onClick}
              disabled={disabled}
              bg={disabled ? "none" : undefined}
            >
              <Icon />
            </ActionIcon>
          </Tooltip>
        ))}
      </Group>
    </Stack>
  );
}

export default DebuggerControls;
