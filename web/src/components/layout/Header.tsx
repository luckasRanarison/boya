import {
  Box,
  Button,
  Group,
  Menu,
  Text,
  ActionIcon,
  ThemeIcon,
} from "@mantine/core";
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
            {views.map(({ name, icon: Icon, ...view }) =>
              "sub" in view ? (
                <Menu key={name} width="175" position="right-start" offset={15}>
                  <Menu.Target>
                    <Menu.Sub.Item
                      leftSection={
                        <ThemeIcon c="indigo" size="sm" variant="transparent">
                          <Icon />
                        </ThemeIcon>
                      }
                    >
                      <Text ml="xs" size="md">
                        {name}
                      </Text>
                    </Menu.Sub.Item>
                  </Menu.Target>
                  <Menu.Dropdown>
                    {view.sub.map(({ icon: Icon, ...sub }) => (
                      <Menu.Item
                        key={sub.name}
                        leftSection={
                          <ThemeIcon c="indigo" size="sm" variant="transparent">
                            <Icon />
                          </ThemeIcon>
                        }
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
                  leftSection={
                    <ThemeIcon c="indigo" size="sm" variant="transparent">
                      <Icon />
                    </ThemeIcon>
                  }
                  onClick={() => setView({ name })}
                  hiddenFrom={"mobileOnly" in view ? "sm" : undefined}
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
