import { Select, Stack } from "@mantine/core";
import {
  usePersistantStore,
  type ColorTheme,
} from "../../../stores/persistantStore";

function SettingsView() {
  const { theme, setTheme } = usePersistantStore();

  return (
    <Stack p="md">
      <Select
        label="Theme"
        value={theme}
        onChange={(e) => e && setTheme(e as ColorTheme)}
        data={["light", "dark"]}
      />
    </Stack>
  );
}

export default SettingsView;
