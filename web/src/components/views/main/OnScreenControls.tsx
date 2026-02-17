import styles from "./OnScreenControls.module.css";
import { keys } from "@/lib/keymap";
import { useRuntimeActions } from "@/stores/runtimeStore";
import { Box, UnstyledButton, Group, Text } from "@mantine/core";
import {
  IconCaretDownFilled,
  IconCaretLeftFilled,
  IconCaretRightFilled,
  IconCaretUpFilled,
  type Icon,
} from "@tabler/icons-react";

function ControlButton({
  button,
  label,
  icon: Icon,
}: {
  button: number;
  label?: string;
  icon?: Icon;
}) {
  const { updateKeypad } = useRuntimeActions();

  const handleStart = () => {
    updateKeypad((prev) => prev & ~button);
  };

  const handleEnd = () => {
    updateKeypad((prev) => prev | button);
  };

  const baseStyle =
    label && label.length > 1
      ? { padding: 5, borderRadius: 4 }
      : { width: 40, height: 40, borderRadius: "50%" };

  return (
    <UnstyledButton
      onPointerDown={handleStart}
      onPointerUp={handleEnd}
      style={{ ...baseStyle }}
      className={styles["gamepad-button"]}
    >
      {Icon ? (
        <Icon size={32} />
      ) : (
        <Text fw={700} size={label && label?.length > 1 ? "xs" : "xl"}>
          {label}
        </Text>
      )}
    </UnstyledButton>
  );
}

export const OnScreenControls = ({ fullscreen }: { fullscreen: boolean }) => (
  <Box
    hiddenFrom="md"
    px="xl"
    pb={fullscreen ? "sm" : 60}
    style={{
      width: "100%",
      position: "absolute",
      bottom: 20,
      pointerEvents: "none",
      zIndex: 100,
    }}
  >
    <Group justify="space-between" align="center" style={{ height: "100%" }}>
      <Box
        style={{
          pointerEvents: "all",
          display: "grid",
          gridTemplateColumns: "repeat(3, 40px)",
        }}
      >
        <Box />
        <ControlButton button={keys.Up} icon={IconCaretUpFilled} />
        <Box />
        <ControlButton button={keys.Left} icon={IconCaretLeftFilled} />
        <Box />
        <ControlButton button={keys.Right} icon={IconCaretRightFilled} />
        <Box />
        <ControlButton button={keys.Down} icon={IconCaretDownFilled} />
      </Box>

      <Group gap="md" style={{ pointerEvents: "all" }}>
        <ControlButton button={keys.B} label="B" />
        <ControlButton button={keys.A} label="A" />
      </Group>
    </Group>

    <Group
      justify="center"
      gap="md"
      style={{
        pointerEvents: "all",
      }}
    >
      <ControlButton button={keys.Select} label="SELECT" />
      <ControlButton button={keys.Start} label="START" />
    </Group>
  </Box>
);

export default OnScreenControls;
