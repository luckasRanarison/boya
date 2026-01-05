import { Stack, Tabs } from "@mantine/core";
import { IconBug, IconInfoCircle, IconSettings } from "@tabler/icons-react";
import CpuView from "../views/CpuView";

function Navbar() {
  return (
    <Stack h="100%" w="100%" justify="space-between">
      <Tabs variant="outline" defaultValue="info">
        <Tabs.List>
          <Tabs.Tab value="info" leftSection={<IconInfoCircle size={14} />}>
            Info
          </Tabs.Tab>
          <Tabs.Tab value="debugger" leftSection={<IconBug size={14} />}>
            Debugger
          </Tabs.Tab>
          <Tabs.Tab value="settings" leftSection={<IconSettings size={14} />}>
            Settings
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="info">info</Tabs.Panel>
        <Tabs.Panel value="debugger">
          <CpuView />
        </Tabs.Panel>
        <Tabs.Panel value="settings">settings</Tabs.Panel>
      </Tabs>
    </Stack>
  );
}

export default Navbar;
