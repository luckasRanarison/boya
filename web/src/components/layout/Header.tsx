import { Box, Button, Group, Menu, Text, ActionIcon } from "@mantine/core";
import {
  IconBrandGithub,
  IconChevronDown,
  IconMenu2,
} from "@tabler/icons-react";
import AppTitle from "./AppTitle";
import { views, useViewStore, useViewActions } from "@/stores/viewStore";

function Header() {
  const view = useViewStore((state) => state.view);
  const { setView } = useViewActions();

  return (
    <Group p="md" h="100%" justify="space-between" align="center">
      <Box px="md" hiddenFrom="sm">
        <AppTitle />
      </Box>

      <Box>
        <Menu width="175" position="bottom-start" offset={30}>
          <Menu.Target>
            <Box>
              <ActionIcon hiddenFrom="sm" variant="transparent">
                <IconMenu2 />
              </ActionIcon>
              <Button
                visibleFrom="sm"
                variant="subtle"
                rightSection={<IconChevronDown />}
                fz="md"
              >
                {view.name}
              </Button>
            </Box>
          </Menu.Target>

          <Menu.Dropdown>
            {views.map(({ name, sub, mobileOnly, icon: Icon }) =>
              sub ? (
                <Menu key={name} width="175" position="right-start" offset={15}>
                  <Menu.Target>
                    <Menu.Sub.Item leftSection={<Icon size={18} />}>
                      <Text ml="xs" size="md">
                        {name}
                      </Text>
                    </Menu.Sub.Item>
                  </Menu.Target>
                  <Menu.Dropdown>
                    {sub.map(({ icon: Icon, ...sub }) => (
                      <Menu.Item
                        key={sub.name}
                        leftSection={<Icon size={18} />}
                        onClick={() => setView({ name, sub })}
                      >
                        <Text ml="xs" size="md">
                          {sub.name}
                        </Text>
                      </Menu.Item>
                    ))}
                  </Menu.Dropdown>
                </Menu>
              ) : (
                <Menu.Item
                  key={name}
                  leftSection={<Icon size={18} />}
                  onClick={() => setView({ name })}
                  hiddenFrom={mobileOnly ? "sm" : undefined}
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
