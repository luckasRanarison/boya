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
import { useMemo } from "react";
import { useDebuggerStore } from "../../../stores/debuggerStore";
import { formatHex } from "@/utils";

function DebuggerControls() {
  const {
    breakpoints,
    romLoaded,
    running,
    pause,
    stepInto,
    setBreakpoints,
    run,
  } = useDebuggerStore();

  const actions = useMemo(
    () => [
      {
        icon: IconRestore,
        label: "Reset",
        onClick: () => console.log("reset"),
        disabled: running || !romLoaded,
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
        icon: running ? IconPlayerPause : IconPlayerPlay,
        label: running ? "Pause" : "Continue",
        onClick: () => (running ? pause() : run()),
        disabled: !romLoaded,
      },
      {
        icon: IconStepInto,
        label: "Step into",
        onClick: () => stepInto(),
        disabled: running || !romLoaded,
      },
      {
        icon: IconStepOut,
        label: "Step out",
        onClick: () => console.log("step out"),
        disabled: running || !romLoaded,
      },
      {
        icon: IconClockPlay,
        label: "Run",
        onClick: () => console.log("run"),
        disabled: running || !romLoaded,
      },
    ],
    [running, romLoaded, stepInto, pause, run],
  );

  const handleBreakpointUpdate = (breakpoints: string[]) => {
    const newBreakpoints = breakpoints
      .map((b) => (b.startsWith("0x") ? parseInt(b.slice(2), 16) : parseInt(b)))
      .filter((b) => !isNaN(b));

    setBreakpoints(newBreakpoints);
  };

  return (
    <Stack mb="md">
      <Group w="100%" justify="center">
        {actions.map(({ icon: Icon, label, disabled, onClick }, i) => (
          <Tooltip key={i} label={label}>
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
        value={breakpoints.map((b) => formatHex(b))}
        variant="filled"
        onChange={handleBreakpointUpdate}
      />
    </Stack>
  );
}

export default DebuggerControls;
