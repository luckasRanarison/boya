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
import HeroView from "./components/views/main/HeroView";
import MemoryView from "./components/views/memory/MemoryView";
import ObjectView from "./components/views/objects/ObjectView";
import BackgroundView from "./components/views/backgrounds/BackgroundView";
import DebuggerView from "./components/views/debugger/DebuggerView";
import SettingsView from "./components/views/settings/SettingsView";
import AboutView from "./components/views/about/AboutView";
import CPURegisterView from "./components/views/registers/CPURegisterView";
import IORegisterView from "./components/views/registers/IORegisterView";

const ROUTES = [
  {
    path: "",
    label: "Dashboard",
    icon: IconLayoutDashboard,
    component: HeroView,
  },
  {
    path: "memory",
    label: "Memory",
    icon: IconStack3,
    component: MemoryView,
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
      {
        path: "cpu",
        label: "CPU",
        icon: IconCpu,
        component: CPURegisterView,
      },
      {
        path: "io",
        label: "I/O",
        icon: IconBlocks,
        component: IORegisterView,
      },
    ],
  },
  {
    path: "objects",
    label: "Objects",
    icon: IconCube,
    component: ObjectView,
  },
  {
    path: "backgrounds",
    label: "Backgrounds",
    icon: IconPhoto,
    component: BackgroundView,
  },
  {
    path: "debugger",
    label: "Debugger",
    icon: IconBug,
    mobileOnly: true,
    component: DebuggerView,
  },
  {
    path: "settings",
    label: "Settings",
    icon: IconSettings,
    mobileOnly: true,
    component: SettingsView,
  },
  {
    path: "about",
    label: "About",
    icon: IconInfoCircle,
    mobileOnly: true,
    component: AboutView,
  },
];

export default ROUTES;
