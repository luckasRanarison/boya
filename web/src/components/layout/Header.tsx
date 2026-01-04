import { Flex, Group, Select, Text } from "@mantine/core";
import { memoryRegions } from "../../lib/gba";
import { useView, type View } from "../../stores/viewStore";

function Header() {
  const { view, setView } = useView();

  return (
    <Group justify="space-between">
      <Flex h="100%" align="center">
        <Text size="xl" c="indigo" fw={700}>
          Bōya
        </Text>
      </Flex>
      <Select
        w="175"
        value={view}
        onChange={(v) => v && setView(v as View)}
        data={["main", ...Object.keys(memoryRegions)]}
      />
    </Group>
  );
}

export default Header;
