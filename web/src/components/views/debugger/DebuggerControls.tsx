import {
  ActionIcon,
  Box,
  Card,
  Flex,
  Group,
  Menu,
  Portal,
  ThemeIcon,
} from "@mantine/core";
import {
  IconArrowBack,
  IconFoldDown,
  IconFoldUp,
  IconFrame,
  IconGripVertical,
  IconPlayerPause,
  IconPlayerPlay,
  IconPlus,
  IconRestore,
  IconSortDescending,
  IconStepInto,
  IconStepOut,
  IconTimelineEventExclamation,
} from "@tabler/icons-react";
import Draggable from "react-draggable";
import { useViewActions, useViewStore } from "@/stores/viewStore";
import { useRef } from "react";
import { useRuntimeStore } from "@/stores/runtimeStore";
import { useDebuggerStore } from "@/stores/debuggerStore";
import {
  useFloatingPositions,
  type Position,
} from "@/hooks/useFloatingPositions";
import { useDebuggerControls } from "@/hooks/useDebuggerControls";

function Wrapper(props: {
  children: React.ReactNode;
  floatConfig?: { position: Position };
}) {
  const nodeRef = useRef<HTMLDivElement>(null);
  const cssPositions = useFloatingPositions(370);

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
  const floatingWindows = useViewStore((state) => state.floatingWindows);
  const running = useRuntimeStore((state) => state.running);
  const romLoaded = useRuntimeStore((state) => state.romLoaded);
  const callstack = useDebuggerStore((state) => state.callstack);
  const floatingPanel = floatingWindows.includes("panel");

  const { toggleWindow } = useViewActions();
  const dbg = useDebuggerControls();

  const controlActions = [
    {
      icon: IconRestore,
      label: "Reset",
      onClick: dbg.reset,
      disabled: running || !romLoaded,
    },
    {
      icon: IconPlus,
      label: "Navigate",
      disabled: running || !romLoaded,
      options: [
        {
          label: "Step Scanline",
          onClick: dbg.stepScanline,
          icon: IconSortDescending,
        },
        {
          label: "Step Frame",
          onClick: dbg.stepFrame,
          icon: IconFrame,
        },
        {
          label: "Step IRQ",
          onClick: dbg.stepIrq,
          icon: IconTimelineEventExclamation,
        },
      ],
    },
    {
      icon: IconArrowBack,
      label: "Step Back",
      onClick: () => alert("Not yet implemented!"),
      disabled: true,
    },
    {
      icon: running ? IconPlayerPause : IconPlayerPlay,
      label: running ? "Pause" : "Continue",
      onClick: dbg.toggleRun,
      disabled: !romLoaded,
    },
    {
      icon: IconStepInto,
      label: "Step into",
      onClick: dbg.stepInto,
      disabled: running || !romLoaded,
    },
    {
      icon: IconStepOut,
      label: "Step out",
      onClick: dbg.stepOut,
      disabled: running || !callstack.length,
    },
    {
      icon: floatingPanel ? IconFoldDown : IconFoldUp,
      label: floatingPanel ? "Dock" : "Detach",
      onClick: () => toggleWindow("panel"),
      disabled: false,
    },
  ];

  return (
    <Wrapper
      floatConfig={props.position ? { position: props.position } : undefined}
    >
      <Group w="100%" align="center" justify="center">
        {floatingPanel && (
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

        {controlActions.map(
          ({ icon: Icon, label, disabled, onClick, options: sub }) => {
            const button = (
              <ActionIcon
                key={label}
                title={label}
                variant="subtle"
                onClick={onClick}
                disabled={disabled}
                bg={disabled ? "none" : undefined}
              >
                <Icon />
              </ActionIcon>
            );

            if (!sub) return button;

            return (
              <Menu key={label} offset={30}>
                <Menu.Target>
                  <Flex>{button}</Flex>
                </Menu.Target>
                <Menu.Dropdown>
                  {sub.map(({ label, icon: Icon, onClick }, i) => (
                    <Menu.Item
                      key={i}
                      leftSection={
                        <ThemeIcon variant="subtle" c="indigo">
                          <Icon size={18} />
                        </ThemeIcon>
                      }
                      onClick={onClick}
                    >
                      {label}
                    </Menu.Item>
                  ))}
                </Menu.Dropdown>
              </Menu>
            );
          },
        )}
      </Group>
    </Wrapper>
  );
}

export default DebuggerControls;
