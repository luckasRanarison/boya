import {
  AppShell,
  Text,
  createTheme,
  Flex,
  MantineProvider,
} from "@mantine/core";
import Navbar from "./components/layout/Navbar";
import Main from "./components/layout/Main";
import Aside from "./components/layout/Aside";

function App() {
  const theme = createTheme({
    primaryColor: "indigo",
  });

  return (
    <MantineProvider theme={theme} defaultColorScheme="light">
      <AppShell
        padding={"xl"}
        header={{ height: 70 }}
        navbar={{ width: 300, breakpoint: "sm" }}
        aside={{ width: 400, breakpoint: "sm" }}
      >
        <AppShell.Header py="md" px="xl">
          <Flex h="100%" align="center">
            <Text size="xl" c="indigo">
              Bōya
            </Text>
          </Flex>
        </AppShell.Header>

        <AppShell.Navbar p="md">
          <Navbar />
        </AppShell.Navbar>

        <AppShell.Main>
          <Main />
        </AppShell.Main>

        <AppShell.Aside p="md">
          <Aside />
        </AppShell.Aside>
      </AppShell>
    </MantineProvider>
  );
}

export default App;
