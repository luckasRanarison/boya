import { getActiveKeys, keyIconMap } from "@/lib/keymap";
import { useBreakpoints } from "@/stores/debuggerStore";
import { useRuntimeActions, useRuntimeStore } from "@/stores/runtimeStore";
import { useViewActions } from "@/stores/viewStore";
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
  const rt = useRuntimeActions();
  const { renderFrame } = useViewActions();
  const fps = useRuntimeStore((state) => state.fps);
  const keypad = useRuntimeStore((state) => state.keypad);
  const running = useRuntimeStore((state) => state.running);
  const breakpoints = useBreakpoints();
  const activeKeys = getActiveKeys(keypad);

  const handleReset = () => {
    rt.reset();
    rt.run({ onFrame: renderFrame, breakpoints });
  };

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
      action: handleReset,
    },
    running
      ? {
          name: "Pause",
          icon: IconPlayerPause,
          action: rt.pause,
        }
      : {
          name: "Continue",
          icon: IconPlayerPlay,
          action: () => rt.run({ onFrame: renderFrame, breakpoints }),
        },
    {
      name: "Stop",
      icon: IconX,
      action: rt.unload,
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
                <Menu.Item
                  key={name}
                  leftSection={<Icon size={16} />}
                  onClick={action}
                >
                  <Text ml="xs" size="md">
                    {name}
                  </Text>
                </Menu.Item>
              ))}
            </Menu.Dropdown>
          </Menu>
          {running ? (
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
            {fps}
            /60 FPS
          </Text>
        </Group>
      </Group>
    </AppShell.Footer>
  );
}

export default EmulatorFooter;
