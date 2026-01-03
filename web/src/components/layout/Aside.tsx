import { Text, Select, Stack } from "@mantine/core";
import { memoryRegions } from "../../lib/gba.ts";

function Aside() {
  return (
    <Stack>
      <Select
        label={
          <Text ff="monospace" size="sm">
            Memory view
          </Text>
        }
        defaultValue={"bios"}
        data={memoryRegions.map((r) => r.name)}
      />
    </Stack>
  );
}

export default Aside;
