import Tile from "@/components/common/Tile";
import { GBA } from "@/lib/gba";
import {
  Box,
  CheckboxIndicator,
  Divider,
  Group,
  Modal,
  SimpleGrid,
  Stack,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import type { Obj } from "boya_wasm";
import { useState } from "react";
import ColorView from "../memory/ColorView";

function ObjectView() {
  const [objId, setObjId] = useState<number | null>(null);
  const [opened, { close, open }] = useDisclosure();
  const objects = GBA.objects();

  return (
    <Group flex={1} py="xl" px="md" justify="center">
      {objId !== null && (
        <ObjectModal
          id={objId}
          object={objects[objId]}
          onClose={close}
          opened={opened}
        />
      )}

      <SimpleGrid cols={{ base: 5, md: 8, lg: 10, xl: 12 }}>
        {objects.map((obj, id) => (
          <Box
            onClick={() => {
              open();
              setObjId(id);
            }}
          >
            <Tile
              render={() => GBA.renderObjBuffer(id)}
              width={60}
              height={60}
              innerWidth={obj.width}
              innerHeight={obj.height}
            />
          </Box>
        ))}
      </SimpleGrid>
    </Group>
  );
}

function ObjectModal(props: {
  id: number;
  object: Obj;
  opened: boolean;
  onClose: () => void;
}) {
  const { object: obj } = props;

  const flags = [
    { label: "transform", value: obj.transform },
    { label: "vflip", value: obj.vflip },
    { label: "hflip", value: obj.hflip },
    { label: "mosaic", value: obj.mosaic },
    { label: "double size", value: obj.double_size },
  ];

  return (
    <Modal
      title={props.id}
      opened={props.opened}
      onClose={props.onClose}
      centered
    >
      <Stack p="xs" gap="xl">
        <Stack flex={1} align="center">
          <Tile
            render={() => GBA.renderObjBuffer(props.id)}
            width={obj.width * 8}
            height={obj.height * 8}
            innerWidth={obj.width}
            innerHeight={obj.height}
          />
          <Group>
            {obj.width.toString()}x{obj.height.toString()}
          </Group>
        </Stack>
        <Stack gap="xs">
          <Group flex={1} justify="center">
            <Group>x: {obj.x.toString()}</Group>
            <Group>y: {obj.y.toString()}</Group>
          </Group>
          <Group>priority: {obj.priority.toString()}</Group>
          <Divider />
          <Group justify="space-between">
            <Stack>
              <Group>
                color mode: {obj.color_mode === 0 ? "16 colors" : "256 colors"}
              </Group>
              {obj.color_mode === 0 && (
                <Group>palette: {obj.palette.toString()}</Group>
              )}
            </Stack>
            <ColorView
              value={Array.from(GBA.getObjectPalette(props.id))}
              columns={4}
            />
          </Group>
          <Divider />
          {flags.map((f) => (
            <Group justify="space-between">
              <Box>{f.label}: </Box>
              <CheckboxIndicator checked={f.value} />
            </Group>
          ))}
        </Stack>
      </Stack>
    </Modal>
  );
}

export default ObjectView;
