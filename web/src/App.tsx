import { AppShell, createTheme, MantineProvider } from "@mantine/core";
import Navbar from "./components/layout/Navbar";
import Main from "./components/layout/Main";
import { Notifications } from "@mantine/notifications";
import Header from "./components/layout/Header";
import { useEffect } from "react";
import { usePersistantStore } from "./stores/persistantStore";
import { GBA } from "./lib/gba";
import { useViewStore } from "./stores/viewStore";
import FloatingPortal from "./components/layout/FloatingPortal";

const mantineTheme = createTheme({
  primaryColor: "indigo",
});

function App() {
  const { bios, theme: colorScheme } = usePersistantStore();
  const floatingWindows = useViewStore((state) => state.floatingWindows);

  useEffect(() => {
    if (bios) {
      GBA.loadBios(bios);
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
          collapsed: { mobile: true },
        }}
      >
        <AppShell.Header>
          <Header />
        </AppShell.Header>

        <AppShell.Navbar display="flex">
          <Navbar />
        </AppShell.Navbar>

        <AppShell.Main display="flex">
          <Main />
        </AppShell.Main>
      </AppShell>

      <Notifications />

      {floatingWindows.length > 0 && <FloatingPortal />}
    </MantineProvider>
  );
}

export default App;
