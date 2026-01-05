import { Group, Text } from "@mantine/core";
import { IconDeviceGamepadFilled } from "@tabler/icons-react";

function AppTitle() {
  return (
    <Group c="indigo">
      <IconDeviceGamepadFilled size={30} />
      <Text size="xl" fw={700} mt="6">
        Bōya
      </Text>
    </Group>
  );
}

export default AppTitle;
