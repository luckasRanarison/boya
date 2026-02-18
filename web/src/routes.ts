import {
  IconArrowsSort,
  IconBlocks,
  IconBug,
  IconCpu,
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

const ROUTES = [
  { path: "", label: "Dashboard", icon: IconLayoutDashboard },
  {
    path: "memory",
    label: "Memory",
    icon: IconStack3,
    sub: [
      { path: "bios", label: "BIOS", icon: IconFileDigit },
      { path: "ewram", label: "EWRAM", icon: IconStack3 },
      { path: "iwram", label: "IWRAM", icon: IconStack2 },
      { path: "palette", label: "Palette", icon: IconPalette },
      { path: "vram", label: "VRAM", icon: IconPhoto },
      { path: "oam", label: "OAM", icon: IconCube },
      { path: "rom", label: "ROM", icon: IconSourceCode },
      { path: "sram", label: "SRAM", icon: IconDatabase },
    ],
  },
  {
    path: "registers",
    label: "Registers",
    icon: IconArrowsSort,
    sub: [
      { path: "cpu", label: "CPU", icon: IconCpu },
      { path: "io", label: "I/O", icon: IconBlocks },
    ],
  },
  { path: "objects", label: "Objects", icon: IconCube },
  { path: "backgrounds", label: "Backgrounds", icon: IconPhoto },
  { path: "debugger", label: "Debugger", icon: IconBug, mobileOnly: true },
  { path: "settings", label: "Settings", icon: IconSettings, mobileOnly: true },
  { path: "about", label: "About", icon: IconInfoCircle, mobileOnly: true },
];

export default ROUTES;
