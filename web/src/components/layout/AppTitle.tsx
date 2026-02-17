import { Group, Text } from "@mantine/core";

function AppTitle() {
  return (
    <Group c="indigo" justify="space-between">
      <Text size="xl" fw="bolder">
        Bōya
      </Text>
      <Text c="dimmed" size="md" fw="bold">
        v{import.meta.env.APP_VERSION}
      </Text>
    </Group>
  );
}

export default AppTitle;
