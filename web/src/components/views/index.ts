import {
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
  { name: "main", icon: IconLayoutDashboard },
  { name: "bios", icon: IconFileDigit },
  { name: "ewram", icon: IconStack3 },
  { name: "iwram", icon: IconStack2 },
  { name: "palette", icon: IconPalette },
  { name: "vram", icon: IconPhoto },
  { name: "oam", icon: IconCube },
  { name: "rom", icon: IconSourceCode },
  { name: "sram", icon: IconDatabase },
  { name: "about", icon: IconInfoCircle, mobileOnly: true },
  { name: "debugger", icon: IconBug, mobileOnly: true },
  { name: "settings", icon: IconSettings, mobileOnly: true },
] as const;

export type View = (typeof views)[number]["name"];
