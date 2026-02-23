import { useRuntimeStore } from "@/stores/runtimeStore";
import { Stack, Text, Card, Group, Badge, Divider, Box } from "@mantine/core";

function AboutView() {
  const romHeader = useRuntimeStore((state) => state.romHeader);

  if (!romHeader)
    return (
      <Stack py="xl" px="md" align="center">
        <Text>🚧 This is a work in progress... 🚧</Text>
      </Stack>
    );

  return (
    <Box py="lg" px="md">
      <Card withBorder radius="md" p="md">
        <Stack gap="xs">
          <Group justify="space-between">
            <Text fw={700} ff="monospace" size="lg">
              {romHeader.title}
            </Text>
            <Badge variant="outline" radius="sm">
              v{romHeader.software_version}
            </Badge>
          </Group>

          <Divider variant="dashed" />

          <Group justify="space-between">
            <Text size="xs" c="dimmed">
              GAME CODE
            </Text>
            <Text c="blue" size="xs">
              {romHeader.game_code}
            </Text>
          </Group>

          <Group justify="space-between">
            <Text size="xs" c="dimmed">
              ENTRY
            </Text>
            <Text size="xs" ff="monospace">
              0x{romHeader.entry_point.toString(16).toUpperCase()}
            </Text>
          </Group>

          <Group justify="space-between">
            <Text size="xs" c="dimmed">
              MAKER
            </Text>
            <Text size="xs">{romHeader.maker_code}</Text>
          </Group>

          <Group justify="space-between">
            <Text size="xs" c="dimmed">
              CHECKSUM
            </Text>
            <Badge color="teal" variant="outline">
              0x{romHeader.checksum.toString(16).toUpperCase()}
            </Badge>
          </Group>
        </Stack>
      </Card>
    </Box>
  );
}

export default AboutView;
