import styles from "./OnScreenControls.module.css";
import { keys } from "@/lib/keymap";
import { useRuntimeActions } from "@/stores/runtimeStore";
import { Box, UnstyledButton, Group, Text, Stack, Flex } from "@mantine/core";
import {
  IconCaretDownFilled,
  IconCaretLeftFilled,
  IconCaretRightFilled,
  IconCaretUpFilled,
  type Icon,
} from "@tabler/icons-react";

const buttonStyleMap = {
  base: { width: 45, height: 45, borderRadius: "50%" },
  meta: { padding: 5, borderRadius: 4 },
  trigger: { padding: 2, borderRadius: 4, width: 80 },
};

function ControlButton({
  button,
  label,
  style = "base",
  icon: Icon,
}: {
  button: number;
  style?: "base" | "trigger" | "meta";
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

  return (
    <UnstyledButton
      onTouchStart={handleStart}
      onTouchEnd={handleEnd}
      style={buttonStyleMap[style]}
      className={styles["gamepad-button"]}
    >
      {Icon ? (
        <Icon size={32} />
      ) : (
        <Text
          fw={700}
          size={style === "meta" ? "xs" : style === "trigger" ? "md" : "xl"}
        >
          {label}
        </Text>
      )}
    </UnstyledButton>
  );
}

export const OnScreenControls = ({ fullscreen }: { fullscreen: boolean }) => (
  <Flex
    hiddenFrom="md"
    direction="column"
    justify={{ base: "end", sm: "space-between" }}
    gap="lg"
    px="xl"
    pt={40}
    pb={fullscreen ? "sm" : 60}
    style={{
      height: "100%",
      width: "100%",
      position: "absolute",
      bottom: 20,
      pointerEvents: "none",
      zIndex: 100,
    }}
  >
    <Group justify="space-between" style={{ pointerEvents: "all" }}>
      <ControlButton button={keys.L} label="L" style="trigger" />
      <ControlButton button={keys.R} label="R" style="trigger" />
    </Group>
    <Stack>
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
        <ControlButton button={keys.Select} label="SELECT" style="meta" />
        <ControlButton button={keys.Start} label="START" style="meta" />
      </Group>
    </Stack>
  </Flex>
);

export default OnScreenControls;
