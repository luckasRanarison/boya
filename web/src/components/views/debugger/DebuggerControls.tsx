import {
  ActionIcon,
  Box,
  Card,
  Group,
  Portal,
  ThemeIcon,
  Tooltip,
} from "@mantine/core";
import {
  IconArrowBack,
  IconArrowBackUp,
  IconFoldDown,
  IconFoldUp,
  IconGripVertical,
  IconPlayerPause,
  IconPlayerPlay,
  IconRestore,
  IconStepInto,
  IconStepOut,
} from "@tabler/icons-react";
import Draggable from "react-draggable";
import { useViewActions, useViewStore } from "@/stores/viewStore";
import { GBA } from "@/lib/gba";
import { useRef } from "react";
import { useMediaQuery } from "@mantine/hooks";
import { floatingPositions, type Position } from "@/utils/float";
import { useRuntimeActions, useRuntimeStore } from "@/stores/runtimeStore";
import { useGotoMemory } from "@/hooks/useGotoMemory";
import { useBreakpoints } from "@/stores/debuggerStore";

function Wrapper(props: {
  children: React.ReactNode;
  floatConfig?: { position: Position };
}) {
  const nodeRef = useRef<HTMLDivElement>(null);
  const isMobile = useMediaQuery("(max-width: 768px)");
  const cssPositions = floatingPositions(isMobile);

  return props.floatConfig ? (
    <Portal>
      <Draggable
        key={props.floatConfig.position} // force re-render on position change
        nodeRef={nodeRef}
        handle=".drag-handle"
      >
        <Card
          ref={nodeRef}
          style={{
            zIndex: 1000,
            position: "fixed",
            ...cssPositions[props.floatConfig.position],
          }}
          withBorder
        >
          {props.children}
        </Card>
      </Draggable>
    </Portal>
  ) : (
    <Box>{props.children}</Box>
  );
}

function DebuggerControls(props: { position?: Position }) {
  const view = useViewStore((state) => state.view);
  const debugPannel = useViewStore((state) => state.debugPannel);
  const running = useRuntimeStore((state) => state.running);
  const romLoaded = useRuntimeStore((state) => state.romLoaded);
  const breakpoints = useBreakpoints();

  const gotoMemory = useGotoMemory();
  const { toggleDebugPannel, renderFrame } = useViewActions();
  const rt = useRuntimeActions();

  const handleReset = () => {
    rt.reset();
    rt.run({ onFrame: renderFrame, breakpoints });
  };

  const handleStepInto = () => {
    rt.stepInto();

    if (view.name === "memory") {
      gotoMemory({
        address: GBA.execAddress(),
        mode: "code",
      });
    }
  };

  const controlActions = [
    {
      icon: IconRestore,
      label: "Reset",
      onClick: handleReset,
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
      onClick: () =>
        running ? rt.pause() : rt.run({ onFrame: renderFrame, breakpoints }),
      disabled: !romLoaded,
    },
    {
      icon: IconStepInto,
      label: "Step into",
      onClick: handleStepInto,
      disabled: running || !romLoaded,
    },
    {
      icon: IconStepOut,
      label: "Step out",
      onClick: () => console.log("step out"),
      disabled: true,
    },
    {
      icon: debugPannel.floating ? IconFoldDown : IconFoldUp,
      label: debugPannel.floating ? "Dock" : "Dettach",
      onClick: toggleDebugPannel,
      disabled: false,
    },
  ];

  return (
    <Wrapper
      floatConfig={props.position ? { position: props.position } : undefined}
    >
      <Group w="100%" align="center" justify="center">
        {debugPannel.floating && (
          <ThemeIcon
            c="gray"
            variant="transparent"
            style={{ cursor: "grab" }}
            onMouseDown={(e) => (e.currentTarget.style.cursor = "grabbing")}
            onMouseUp={(e) => (e.currentTarget.style.cursor = "grab")}
            className="drag-handle"
          >
            <IconGripVertical />
          </ThemeIcon>
        )}

        {controlActions.map(({ icon: Icon, label, disabled, onClick }) => (
          <Tooltip
            offset={props.position ? 25 : undefined}
            key={label}
            label={label}
          >
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
    </Wrapper>
  );
}

export default DebuggerControls;
