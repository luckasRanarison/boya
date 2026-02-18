import Tile from "@/components/common/Tile";
import { Text, Group, SimpleGrid, Stack } from "@mantine/core";
import { useState } from "react";
import { useGba } from "@/hooks/useGba";
import ObjectModal from "./ObjectModal";
import { GBA } from "@/lib/gba";

function ObjectView() {
  const [objId, setObjId] = useState<number | null>(null);
  const { memory } = useGba();
  const objects = memory.getObjects();

  return (
    <Group flex={1} p="md" justify="center">
      {objId !== null && (
        <ObjectModal
          id={objId}
          object={objects[objId]}
          onClose={() => setObjId(null)}
          opened
        />
      )}

      <SimpleGrid cols={{ base: 5, md: 8, lg: 10, xl: 12 }} spacing="md" p="md">
        {objects.map((obj, id) => (
          <Stack gap="xs" align="center" onClick={() => setObjId(id)}>
            <Tile
              render={() => GBA.renderObj(id)}
              width={60}
              height={60}
              innerWidth={obj.width}
              innerHeight={obj.height}
              checkerboard
            />
            <Text c="gray" fw="bold" ff="monospace" size="sm">
              {id.toString().padStart(3, "0")}
            </Text>
          </Stack>
        ))}
      </SimpleGrid>
    </Group>
  );
}

export default ObjectView;
