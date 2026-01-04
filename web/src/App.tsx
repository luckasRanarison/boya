import { AppShell, createTheme, MantineProvider } from "@mantine/core";
import Navbar from "./components/layout/Navbar";
import Main from "./components/layout/Main";
import { Notifications } from "@mantine/notifications";
import Header from "./components/layout/Header";
import { useEffect } from "react";
import { usePersistantStore } from "./stores/persistantStore";
import { instance } from "./lib/gba";

function App() {
  const theme = createTheme({
    primaryColor: "indigo",
  });

  const { bios } = usePersistantStore();

  useEffect(() => {
    if (bios) {
      instance.loadBios(bios);
    }
  }, [bios]);

  return (
    <MantineProvider theme={theme} defaultColorScheme="light">
      <AppShell
        header={{ height: 70 }}
        navbar={{
          width: 350,
          breakpoint: "sm",
          collapsed: { mobile: true },
        }}
      >
        <AppShell.Header py="md" px="xl">
          <Header />
        </AppShell.Header>

        <AppShell.Navbar p="md">
          <Navbar />
        </AppShell.Navbar>

        <AppShell.Main display="flex">
          <Main />
        </AppShell.Main>
      </AppShell>

      <Notifications />
    </MantineProvider>
  );
}

export default App;
