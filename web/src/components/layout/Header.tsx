import { Box, Button, Group, Menu, Text, ActionIcon } from "@mantine/core";
import {
  IconBrandGithub,
  IconChevronDown,
  IconMenu2,
} from "@tabler/icons-react";
import AppTitle from "./AppTitle";
import { views, type View } from "../views";

type Props = {
  view: View;
  navbarOpened: boolean;
  onViewChange: (value: View) => void;
  onNavbarToggle: () => void;
};

function Header(props: Props) {
  return (
    <Group p="md" h="100%" justify="space-between" align="center">
      <Box px="md" hiddenFrom="sm">
        <AppTitle />
      </Box>

      <Box hiddenFrom="sm">
        <ActionIcon variant="transparent" onClick={props.onNavbarToggle}>
          <IconMenu2 />
        </ActionIcon>
      </Box>

      <Box visibleFrom="sm">
        <Menu width="175" position="bottom-start" offset={30}>
          <Menu.Target>
            <Button variant="subtle" rightSection={<IconChevronDown />} fz="md">
              {props.view}
            </Button>
          </Menu.Target>

          <Menu.Dropdown>
            {views.map(
              ({ name, icon: Icon, ...rest }) =>
                !("mobileOnly" in rest) && (
                  <Menu.Item
                    key={name}
                    leftSection={<Icon size={18} />}
                    onClick={() => props.onViewChange(name)}
                  >
                    <Text ml="xs" size="md">
                      {name}
                    </Text>
                  </Menu.Item>
                ),
            )}
          </Menu.Dropdown>
        </Menu>
      </Box>

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
