import { Switch, type SwitchProps } from "@mantine/core";

function SettingsSwitch(props: SwitchProps) {
  return (
    <Switch
      w="100%"
      labelPosition="left"
      styles={{
        body: {
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        },
      }}
      {...props}
    />
  );
}

export default SettingsSwitch;
