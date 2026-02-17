import { useFloatingPositions } from "@/hooks/useFloatingPositions";
import {
  ActionIcon,
  Box,
  CloseButton,
  Group,
  Paper,
  Portal,
  Text,
} from "@mantine/core";
import { IconChevronDown, IconChevronUp } from "@tabler/icons-react";
import { useRef, useState } from "react";
import Draggable from "react-draggable";

function FloatingWindow(props: {
  title: string;
  children: React.ReactNode;
  onClose: () => void;
}) {
  const parentRef = useRef<HTMLDivElement>(null);
  const cssPositions = useFloatingPositions(350);
  const [zIndex, setZIndex] = useState(200);
  const [collapsed, setCollapsed] = useState(false);

  return (
    <Portal>
      <Draggable
        nodeRef={parentRef}
        handle=".drag-handle"
        cancel=".drag-cancel"
        bounds="body"
        onStart={() => setZIndex(1000)}
        onStop={() => setZIndex(200)}
      >
        <Paper
          ref={parentRef}
          withBorder
          style={{
            ...cssPositions["up-right"],
            position: "fixed",
            zIndex,
            minWidth: 350,
            maxHeight: "60vh",
            display: "flex",
            flexDirection: "column",
            backgroundColor: "var(--mantine-color-body)",
          }}
        >
          <Group
            className="drag-handle"
            p="xs"
            justify="space-between"
            wrap="nowrap"
            style={{
              cursor: "grab",
              borderBottom: "1px solid var(--mantine-color-default-border)",
            }}
          >
            <Text size="xs" fw="bold" ff="monospace" c="dimmed" tt="uppercase">
              {props.title}
            </Text>
            <Group>
              <ActionIcon
                title={collapsed ? "Expand" : "Minimize"}
                className="drag-cancel"
                size="sm"
                variant="subtle"
                onClick={() => setCollapsed((prev) => !prev)}
              >
                {collapsed ? <IconChevronDown /> : <IconChevronUp />}
              </ActionIcon>
              <CloseButton
                title="Close"
                className="drag-cancel"
                size="sm"
                variant="subtle"
                onClick={props.onClose}
                c="red"
              />
            </Group>
          </Group>

          {!collapsed && (
            <Box flex={1} ff="monospace" fz="sm" style={{ overflow: "scroll" }}>
              {props.children}
            </Box>
          )}
        </Paper>
      </Draggable>
    </Portal>
  );
}

export default FloatingWindow;
