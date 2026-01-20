import { Box, Stack, Tabs } from "@mantine/core";
import { IconBug, IconInfoCircle, IconSettings } from "@tabler/icons-react";
import AppTitle from "./AppTitle";
import { useState } from "react";
import SettingsView from "../views/settings/SettingsView";
import DebuggerView from "../views/debugger/DebuggerView";

function DesktopNavbar() {
  // use controlled state to load the debugger lazily and improve perf
  const [activeTab, setActiveTab] = useState("about");

  return (
    <Stack h="100%" w="100%" visibleFrom="sm">
      <Box py="md" px="xl">
        <AppTitle />
      </Box>
      <Tabs
        h="100%"
        variant="outline"
        value={activeTab}
        onChange={(value) => value && setActiveTab(value)}
      >
        <Tabs.List h="50">
          <Tabs.Tab value="about" leftSection={<IconInfoCircle size={14} />}>
            About
          </Tabs.Tab>
          <Tabs.Tab value="debugger" leftSection={<IconBug size={14} />}>
            Debugger
          </Tabs.Tab>
          <Tabs.Tab value="settings" leftSection={<IconSettings size={14} />}>
            Settings
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="about">
          <div></div>
        </Tabs.Panel>

        {activeTab === "settings" && (
          <Tabs.Panel value="settings">
            <SettingsView />
          </Tabs.Panel>
        )}

        {activeTab === "debugger" && (
          <Tabs.Panel value="debugger">
            <DebuggerView />
          </Tabs.Panel>
        )}
      </Tabs>
    </Stack>
  );
}

export default DesktopNavbar;
