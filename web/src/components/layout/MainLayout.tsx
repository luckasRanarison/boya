import { AppShell, createTheme, MantineProvider } from "@mantine/core";
import Navbar from "@/components/layout/Navbar";
import { Notifications } from "@mantine/notifications";
import Header from "@/components/layout/Header";
import { useEffect } from "react";
import { usePersistantStore } from "@/stores/persistantStore";
import { useViewStore } from "@/stores/viewStore";
import FloatingPortal from "@/components/layout/FloatingPortal";
import { GBA } from "@/lib/gba";
import { useKeyHandler } from "@/hooks/useKeyHandler";
import { Outlet } from "react-router";

const mantineTheme = createTheme({
  primaryColor: "indigo",
});

function MainLayout() {
  const { bios, theme: colorScheme } = usePersistantStore();
  const floatingWindows = useViewStore((state) => state.floatingWindows);
  const handleKey = useKeyHandler();

  useEffect(() => {
    if (bios) {
      GBA.loadBios(bios);
    }
  }, [bios]);

  useEffect(() => {
    document.addEventListener("keydown", handleKey);
    document.addEventListener("keyup", handleKey);
    return () => {
      document.removeEventListener("keydown", handleKey);
      document.removeEventListener("keyup", handleKey);
    };
  }, [handleKey]);

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
          <Outlet />
        </AppShell.Main>
      </AppShell>

      <Notifications />

      {floatingWindows.length > 0 && <FloatingPortal />}
    </MantineProvider>
  );
}

export default MainLayout;
