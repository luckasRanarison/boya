import Tile from "@/components/common/Tile";
import { GBA } from "@/lib/gba";
import {
  Badge,
  Box,
  Group,
  Modal,
  Paper,
  SimpleGrid,
  Stack,
  Text,
  ThemeIcon,
} from "@mantine/core";
import {
  IconArrowsMaximize,
  IconFlipHorizontal,
  IconFlipVertical,
  IconFocus2,
  IconGrid4x4,
  IconLayersIntersect,
  IconPalette,
} from "@tabler/icons-react";
import type { Obj } from "boya_wasm";
import ColorView from "../memory/ColorView";

function ObjectModal(props: {
  id: number;
  object: Obj;
  opened: boolean;
  onClose: () => void;
}) {
  const { object: obj } = props;

  const flags = [
    {
      label: "Transform",
      value: obj.transform,
      icon: <IconFocus2 size={14} />,
    },
    {
      label: "V-Flip",
      value: obj.vflip,
      icon: <IconFlipVertical size={14} />,
    },
    {
      label: "H-Flip",
      value: obj.hflip,
      icon: <IconFlipHorizontal size={14} />,
    },
    {
      label: "Mosaic",
      value: obj.mosaic,
      icon: <IconGrid4x4 size={14} />,
    },
    {
      label: "Double",
      value: obj.double_size,
      icon: <IconArrowsMaximize size={14} />,
    },
  ];

  return (
    <Modal
      opened={props.opened}
      onClose={props.onClose}
      title={<Text fw={700}>#{props.id}</Text>}
      centered
      size="md"
      radius="md"
    >
      <Stack gap="lg">
        <Stack align="center" gap="xs">
          <Tile
            render={() => GBA.renderObjBuffer(props.id)}
            width={obj.width * 8}
            height={obj.height * 8}
            innerWidth={obj.width}
            innerHeight={obj.height}
            checkerboard
          />
          <Badge variant="light" color="gray" size="sm">
            {obj.width * 8} × {obj.height * 8} px
          </Badge>
        </Stack>

        <SimpleGrid cols={2} spacing="sm">
          <Paper withBorder p="xs" radius="sm">
            <Group gap="xs">
              <ThemeIcon variant="light" color="blue">
                <IconArrowsMaximize size={18} />
              </ThemeIcon>
              <Box>
                <Text size="xs" c="dimmed">
                  Position
                </Text>
                <Text fw={500} size="sm">
                  X: {obj.x} Y: {obj.y}
                </Text>
              </Box>
            </Group>
          </Paper>

          <Paper withBorder p="xs" radius="sm">
            <Group gap="xs">
              <ThemeIcon variant="light" color="grape">
                <IconLayersIntersect size={18} />
              </ThemeIcon>
              <Box>
                <Text size="xs" c="dimmed">
                  Priority
                </Text>
                <Text fw={500} size="sm">
                  Level {obj.priority}
                </Text>
              </Box>
            </Group>
          </Paper>
        </SimpleGrid>

        <Paper withBorder p="xs">
          <Stack gap="sm">
            <Group justify="space-between" mb="xs">
              <Group gap="xs">
                <ThemeIcon variant="light" color="orange">
                  <IconPalette size={18} />
                </ThemeIcon>
                <Box>
                  <Text size="xs" c="dimmed">
                    Palette
                  </Text>
                  <Text fw={500} size="sm">
                    ({obj.color_mode === 0 ? "16" : "256"} colors)
                  </Text>
                </Box>
              </Group>
              {obj.color_mode === 0 && (
                <Badge variant="dot">Index {obj.palette}</Badge>
              )}
            </Group>
            <ColorView
              value={Array.from(GBA.getObjectPalette(props.id))}
              columns={16}
            />
          </Stack>
        </Paper>

        <Stack p="xs">
          <Text size="xs" fw="bold" tt="uppercase" c="dimmed">
            Attributes
          </Text>
          <Group gap="xs">
            {flags.map((f) => (
              <Badge
                key={f.label}
                variant={f.value ? "filled" : "outline"}
                color={f.value ? "blue" : "gray"}
                leftSection={f.icon}
              >
                {f.label}
              </Badge>
            ))}
          </Group>
        </Stack>
      </Stack>
    </Modal>
  );
}

export default ObjectModal;
