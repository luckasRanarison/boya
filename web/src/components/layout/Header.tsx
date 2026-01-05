import { Box, Button, Group, Menu, Text, ActionIcon } from "@mantine/core";
import { IconBrandGithub, IconChevronDown } from "@tabler/icons-react";
import AppTitle from "../AppTitle";
import { views, type View } from "../views";

type Props = {
  view: View;
  onViewChange: (value: View) => void;
};

function Header(props: Props) {
  return (
    <Group p="md" h="100%" justify="space-between" align="center">
      <Box px="md" hiddenFrom="sm">
        <AppTitle />
      </Box>
      <Menu width="150" position="bottom-start" offset={30}>
        <Menu.Target>
          <Button
            c="dark"
            variant="subtle"
            rightSection={<IconChevronDown />}
            fz="md"
          >
            {props.view}
          </Button>
        </Menu.Target>

        <Menu.Dropdown>
          {views.map((v) => (
            <Menu.Item
              key={v.name}
              leftSection={<v.icon size={18} />}
              onClick={() => props.onViewChange(v.name)}
              hiddenFrom={"mobileOnly" in v ? "sm" : undefined}
            >
              <Text size="md">{v.name}</Text>
            </Menu.Item>
          ))}
        </Menu.Dropdown>
      </Menu>
      <ActionIcon
        visibleFrom="md"
        component="a"
        href="https://github.com/luckasranarison/boya"
        variant="subtle"
        aria-label="GitHub"
      >
        <IconBrandGithub />
      </ActionIcon>
    </Group>
  );
}

export default Header;
