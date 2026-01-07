import { ActionIcon, Box, Button, Group, Stack, Text } from "@mantine/core";
import { views, type View } from "../views";
import { IconMenu2, IconX } from "@tabler/icons-react";

type Props = {
  view: View;
  onViewChange: (view: View) => void;
  onNavbarToggle: () => void;
};

function MobileNavbar(props: Props) {
  return (
    <Stack
      h="100%"
      p="md"
      gap="md"
      hiddenFrom="sm"
      style={{ overflow: "scroll" }}
    >
      <Group w="100%" align="center" justify="space-between">
        <Box px="md">
          <Group>
            <IconMenu2 size={25} />
            <Text size="xl" fw={700}>
              Menu
            </Text>
          </Group>
        </Box>
        <ActionIcon
          variant="transparent"
          c="red"
          onClick={props.onNavbarToggle}
        >
          <IconX />
        </ActionIcon>
      </Group>

      <Stack flex={1} justify="center">
        {views.map(({ name, icon: Icon }) => (
          <Button
            key={name}
            w="100%"
            size="md"
            variant={props.view === name ? undefined : "subtle"}
            onClick={() => {
              props.onViewChange(name);
              props.onNavbarToggle();
            }}
            leftSection={
              <Box mr="sm">
                <Icon size={18} />
              </Box>
            }
            styles={{
              label: { width: "100%" },
            }}
          >
            {name}
          </Button>
        ))}
      </Stack>
    </Stack>
  );
}

export default MobileNavbar;
