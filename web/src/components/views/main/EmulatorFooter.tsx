import { getActiveKeys, keyIconMap } from "@/lib/keymap";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { AppShell, Card, Group, Text } from "@mantine/core";

function EmulatorFooter() {
  const { running, fps, keypad } = useDebuggerStore();
  const activeKeys = getActiveKeys(keypad);

  return (
    <AppShell.Footer p="md">
      <Group justify="space-between">
        {running ? (
          <Text c="green">Running</Text>
        ) : (
          <Text c="yellow">Paused</Text>
        )}
        <Group gap="xs">
          {activeKeys.map((key) => {
            const Icon = keyIconMap[key];
            return (
              <Card
                key={key}
                withBorder
                py={1}
                px={5}
                h="20px"
                fz="xs"
                fw={550}
              >
                {Icon ? <Icon size={15} strokeWidth={2.5} /> : key}
              </Card>
            );
          })}
        </Group>
        <Group>
          <Text c="gray">
            {fps}
            /60 FPS
          </Text>
        </Group>
      </Group>
    </AppShell.Footer>
  );
}

export default EmulatorFooter;
