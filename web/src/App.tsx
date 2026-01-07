import { AppShell, createTheme, MantineProvider } from "@mantine/core";
import DesktopNavbar from "./components/layout/DesktopNavbar";
import Main from "./components/layout/Main";
import { Notifications } from "@mantine/notifications";
import Header from "./components/layout/Header";
import { useEffect, useState } from "react";
import { usePersistantStore } from "./stores/persistantStore";
import { instance } from "./lib/gba";
import type { View } from "./components/views";
import { useDisclosure } from "@mantine/hooks";
import MobileNavbar from "./components/layout/MobileNavbar";

function App() {
  const mantineTheme = createTheme({
    primaryColor: "indigo",
  });

  const [view, setView] = useState<View>("main");
  const [opened, { toggle }] = useDisclosure();
  const { bios, theme: colorScheme } = usePersistantStore();

  useEffect(() => {
    if (bios) {
      instance.loadBios(bios);
    }
  }, [bios]);

  return (
    <MantineProvider theme={mantineTheme} forceColorScheme={colorScheme}>
      <AppShell
        layout="alt"
        header={{ height: 70 }}
        navbar={{
          width: 350,
          breakpoint: "sm",
          collapsed: { mobile: !opened },
        }}
      >
        <AppShell.Header>
          <Header
            view={view}
            navbarOpened={opened}
            onViewChange={setView}
            onNavbarToggle={toggle}
          />
        </AppShell.Header>

        <AppShell.Navbar display="flex">
          <DesktopNavbar />
          <MobileNavbar
            view={view}
            onViewChange={setView}
            onNavbarToggle={toggle}
          />
        </AppShell.Navbar>

        <AppShell.Main display="flex">
          <Main view={view} />
        </AppShell.Main>
      </AppShell>

      <Notifications />
    </MantineProvider>
  );
}

export default App;
