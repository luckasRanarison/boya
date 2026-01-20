import {
  IconArrowsSort,
  IconBug,
  IconCube,
  IconDatabase,
  IconFileDigit,
  IconInfoCircle,
  IconLayoutDashboard,
  IconPalette,
  IconPhoto,
  IconSettings,
  IconSourceCode,
  IconStack2,
  IconStack3,
} from "@tabler/icons-react";

export const views = [
  { name: "Main", icon: IconLayoutDashboard },
  {
    name: "Memory",
    icon: IconStack3,
    sub: [
      { name: "bios", icon: IconFileDigit },
      { name: "ewram", icon: IconStack3 },
      { name: "iwram", icon: IconStack2 },
      { name: "palette", icon: IconPalette },
      { name: "vram", icon: IconPhoto },
      { name: "oam", icon: IconCube },
      { name: "rom", icon: IconSourceCode },
      { name: "sram", icon: IconDatabase },
    ],
  },
  { name: "Registers", icon: IconArrowsSort },
  { name: "About", icon: IconInfoCircle, mobileOnly: true },
  { name: "Debugger", icon: IconBug, mobileOnly: true },
  { name: "Settings", icon: IconSettings, mobileOnly: true },
] as const;

export type View = (typeof views)[number]["name"];

export type MenuView = {
  name: View;
  sub?: string;
};
