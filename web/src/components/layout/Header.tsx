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
import { Link } from "react-router";
import { useActiveRoute } from "@/hooks/useActiveRoute";
import ROUTES from "@/routes";

function Header() {
  const { route } = useActiveRoute();

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
                {route?.label}
              </Button>
            </Box>
          </Menu.Target>

          <Menu.Dropdown>
            {ROUTES.map(({ icon: Icon, ...route }) =>
              route.sub ? (
                <Menu
                  key={route.path}
                  width="175"
                  position="right-start"
                  offset={15}
                >
                  <Menu.Target>
                    <Menu.Sub.Item
                      leftSection={
                        <ThemeIcon c="indigo" size="sm" variant="transparent">
                          <Icon />
                        </ThemeIcon>
                      }
                    >
                      <Text ml="xs" size="md">
                        {route.label}
                      </Text>
                    </Menu.Sub.Item>
                  </Menu.Target>
                  <Menu.Dropdown>
                    {route.sub.map(({ icon: Icon, ...sub }) => (
                      <Menu.Item
                        key={sub.path}
                        component={Link}
                        to={`${route.path}/${sub.path}`}
                        leftSection={
                          <ThemeIcon c="indigo" size="sm" variant="transparent">
                            <Icon />
                          </ThemeIcon>
                        }
                      >
                        <Text ml="xs" size="sm">
                          {sub.label}
                        </Text>
                      </Menu.Item>
                    ))}
                  </Menu.Dropdown>
                </Menu>
              ) : (
                <Menu.Item
                  key={route.path}
                  component={Link}
                  to={route.path}
                  leftSection={
                    <ThemeIcon c="indigo" size="sm" variant="transparent">
                      <Icon />
                    </ThemeIcon>
                  }
                  hiddenFrom={route.mobileOnly ? "sm" : undefined}
                >
                  <Text ml="xs" size="md">
                    {route.label}
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
