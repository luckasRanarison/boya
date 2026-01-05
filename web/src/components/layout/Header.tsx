import { Box, Button, Group, Menu, Text, ActionIcon } from "@mantine/core";
import {
  IconBrandGithub,
  IconBug,
  IconChevronDown,
  IconCube,
  IconFileDigit,
  IconInfoCircle,
  IconLayoutDashboard,
  IconPalette,
  IconPhoto,
  IconSettings,
  IconStack2,
  IconStack3,
} from "@tabler/icons-react";
import type { View } from "../../App";
import AppTitle from "../AppTitle";

const views = [
  { name: "main", icon: IconLayoutDashboard },
  { name: "bios", icon: IconFileDigit },
  { name: "ewram", icon: IconStack3 },
  { name: "iwram", icon: IconStack2 },
  { name: "palette", icon: IconPalette },
  { name: "vram", icon: IconPhoto },
  { name: "oam", icon: IconCube },
  { name: "about", icon: IconInfoCircle, mobileOnly: true },
  { name: "debugger", icon: IconBug, mobileOnly: true },
  { name: "settings", icon: IconSettings, mobileOnly: true },
] as const;

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
