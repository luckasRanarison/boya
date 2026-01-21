import { getActiveKeys, keyIconMap } from "@/lib/keymap";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { ActionIcon, AppShell, Card, Group, Menu, Text } from "@mantine/core";
import {
  IconDotsVertical,
  IconPhoto,
  IconPlayerPause,
  IconPlayerPlay,
  IconRestore,
  IconX,
} from "@tabler/icons-react";

function EmulatorFooter(props: { canvas: () => HTMLCanvasElement | null }) {
  const dbg = useDebuggerStore();
  const activeKeys = getActiveKeys(dbg.keypad);

  const handleScreenshot = () => {
    const canvas = props.canvas();

    if (!canvas) return;

    const canvasUrl = canvas.toDataURL("image/png");
    const link = document.createElement("a");

    link.href = canvasUrl;
    link.download = `boay-screenshot-${Date.now()}`;
    link.click();
    link.remove();
  };

  const menuItems = [
    {
      name: "Screenshot",
      icon: IconPhoto,
      action: handleScreenshot,
    },
    {
      name: "Restart",
      icon: IconRestore,
      action: dbg.reset,
    },
    dbg.running
      ? {
          name: "Pause",
          icon: IconPlayerPause,
          action: dbg.pause,
        }
      : {
          name: "Continue",
          icon: IconPlayerPlay,
          action: dbg.run,
        },
    {
      name: "Stop",
      icon: IconX,
      action: dbg.unloadRom,
    },
  ];

  return (
    <AppShell.Footer p="md">
      <Group justify="space-between">
        <Group>
          <Menu position="top-start" width="150" offset={30}>
            <Menu.Target>
              <ActionIcon variant="subtle" c="gray">
                <IconDotsVertical />
              </ActionIcon>
            </Menu.Target>

            <Menu.Dropdown>
              {menuItems.map(({ icon: Icon, name, action }) => (
                <Menu.Item leftSection={<Icon size={16} />} onClick={action}>
                  <Text ml="xs" size="md">
                    {name}
                  </Text>
                </Menu.Item>
              ))}
            </Menu.Dropdown>
          </Menu>
          {dbg.running ? (
            <Text c="green">Running</Text>
          ) : (
            <Text c="yellow">Paused</Text>
          )}
        </Group>

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
            {dbg.fps}
            /60 FPS
          </Text>
        </Group>
      </Group>
    </AppShell.Footer>
  );
}

export default EmulatorFooter;
