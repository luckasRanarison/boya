import { Box, Stack, Tabs } from "@mantine/core";
import { IconBug, IconInfoCircle, IconSettings } from "@tabler/icons-react";
import DebuggerView from "../views/DebuggerView";
import AppTitle from "../AppTitle";

function Navbar() {
  return (
    <Stack h="100%" w="100%">
      <Box py="md" px="xl">
        <AppTitle />
      </Box>
      <Tabs h="100%" variant="outline" defaultValue="debugger">
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

        <Tabs.Panel value="debugger">
          <DebuggerView />
        </Tabs.Panel>

        <Tabs.Panel value="settings">
          <div></div>
        </Tabs.Panel>
      </Tabs>
    </Stack>
  );
}

export default Navbar;
