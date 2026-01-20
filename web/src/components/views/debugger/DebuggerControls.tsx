import { ActionIcon, Group, Stack, TagsInput, Tooltip } from "@mantine/core";
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
import { formatHex } from "@/utils";

function DebuggerControls() {
  const dbg = useDebuggerStore();

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
      onClick: () => dbg.stepInto(),
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

  const handleBreakpointUpdate = (breakpoints: string[]) => {
    const newBreakpoints = breakpoints
      .map((b) => (b.startsWith("0x") ? parseInt(b.slice(2), 16) : parseInt(b)))
      .filter((b) => !isNaN(b));

    dbg.setBreakpoints(newBreakpoints);
  };

  return (
    <Stack mb="md">
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
      <TagsInput
        label="breakpoints"
        placeholder="Enter breakpoint address..."
        value={dbg.breakpoints.map((b) => formatHex(b))}
        variant="filled"
        onChange={handleBreakpointUpdate}
      />
    </Stack>
  );
}

export default DebuggerControls;
