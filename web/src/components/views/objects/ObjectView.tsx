import { GBA } from "@/lib/gba";
import { Card, Group, SimpleGrid, Stack, Tooltip } from "@mantine/core";

function ObjectView() {
  const objects = GBA.objects();

  return (
    <Group flex={1} p="md" justify="center">
      <SimpleGrid cols={12}>
        {objects.map((obj, id) => (
          <Tooltip
            key={id}
            label={
              <Stack gap="xs">
                <Group>x: {obj.x.toString()}</Group>
                <Group>y: {obj.y.toString()}</Group>
                <Group>width: {obj.width.toString()}</Group>
                <Group>height: {obj.height.toString()}</Group>
                <Group>priority: {obj.priority.toString()}</Group>
                <Group>transform: {obj.transform.toString()}</Group>
                <Group>vflip: {obj.vflip.toString()}</Group>
                <Group>hflip: {obj.hflip.toString()}</Group>
                <Group>
                  color mode:{" "}
                  {obj.color_mode === 0 ? "16 colors" : "256 colors"}
                </Group>
                {obj.color_mode === 0 && (
                  <Group>palette: {obj.palette.toString()}</Group>
                )}
                <Group>mosaic: {obj.mosaic.toString()}</Group>
                <Group>double size: {obj.double_size.toString()}</Group>
              </Stack>
            }
          >
            <Card ta="center">{id}</Card>
          </Tooltip>
        ))}
      </SimpleGrid>
    </Group>
  );
}

export default ObjectView;
