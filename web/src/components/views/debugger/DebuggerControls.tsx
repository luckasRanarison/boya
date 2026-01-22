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
import { useView } from "@/stores/viewStore";
import { instance } from "@/lib/gba";
import { useRef } from "react";
import { useMediaQuery } from "@mantine/hooks";
import { floatingPositions, type Position } from "@/utils/float";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { useGotoMemory } from "@/hooks/useGotoMemory";

function Wrapper(props: {
  children: React.ReactNode;
  floatConfig?: { position: Position };
}) {
  const nodeRef = useRef<HTMLDivElement>(null);
  const isMobile = useMediaQuery("(max-width: 768px)");
  const cssPositions = floatingPositions(isMobile);

  return props.floatConfig ? (
    <Portal>
      <Draggable nodeRef={nodeRef} handle=".drag-handle">
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
  const dbg = useDebuggerStore();
  const gotoMemory = useGotoMemory();
  const { view } = useView();

  const handleStepInto = () => {
    dbg.stepInto();

    if (view.name === "memory") {
      gotoMemory({
        address: instance.execAddress(),
        mode: "code",
      });
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
      icon: dbg.panel.floating ? IconFoldDown : IconFoldUp,
      label: dbg.panel.floating ? "Dock" : "Dettach",
      onClick: () => dbg.panel.toggleFloat(),
      disabled: false,
    },
  ];

  return (
    <Wrapper
      floatConfig={props.position ? { position: props.position } : undefined}
    >
      <Group w="100%" align="center" justify="center">
        {dbg.panel.floating && (
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

        {actions.map(({ icon: Icon, label, disabled, onClick }) => (
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
