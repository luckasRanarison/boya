import {
  Modal,
  Stack,
  SimpleGrid,
  Paper,
  Group,
  ThemeIcon,
  Box,
  Text,
  Badge,
} from "@mantine/core";
import {
  IconLayersIntersect,
  IconPalette,
  IconMap2,
  IconGrid4x4,
  IconSquareOff,
} from "@tabler/icons-react";
import Tile from "@/components/common/Tile";
import { GBA } from "@/lib/gba";

const sizeMap = ["256x256", "512x256", "256x512", "512x512"];

function BackgroundModal(props: {
  id: number;
  bgcnt: number;
  onClose: () => void;
}) {
  const { bgcnt, id } = props;

  const priority = bgcnt & 0x3;
  const charBaseBlock = (bgcnt >> 2) & 0x3;
  const mosaic = (bgcnt >> 6) & 0x1;
  const displayAreaOverfloa = (bgcnt >> 13) & 0x1;
  const colorMode = (bgcnt >> 7) & 0x1;
  const screenBaseBlock = (bgcnt >> 8) & 0x1f;
  const screenSize = (bgcnt >> 14) & 0x3;

  const flags = [
    {
      label: "Mosaic",
      value: mosaic,
      icon: <IconGrid4x4 size={14} />,
    },
    {
      label: "Overflow display",
      value: displayAreaOverfloa,
      icon: <IconSquareOff size={14} />,
    },
  ];

  return (
    <Modal
      opened={true}
      onClose={props.onClose}
      title={<Text fw="bold">BG {id}</Text>}
      centered
      size="md"
      radius="md"
    >
      <Stack gap="lg">
        <Stack align="center" gap="xs">
          <Tile
            render={() => GBA.renderBg(id)}
            width={240}
            height={160}
            innerWidth={240}
            innerHeight={160}
            checkerboard
          />
          <Badge variant="light" color="gray" size="sm">
            {sizeMap[screenSize]} px
          </Badge>
        </Stack>

        <SimpleGrid cols={2} spacing="sm">
          <Paper withBorder p="xs" radius="sm">
            <Group gap="xs">
              <ThemeIcon variant="light" color="blue">
                <IconLayersIntersect size={18} />
              </ThemeIcon>
              <Box>
                <Text size="xs" c="dimmed">
                  Priority
                </Text>
                <Text fw={500} size="sm">
                  Level {priority}
                </Text>
              </Box>
            </Group>
          </Paper>

          <Paper withBorder p="xs" radius="sm">
            <Group gap="xs">
              <ThemeIcon variant="light" color="green">
                <IconMap2 size={18} />
              </ThemeIcon>
              <Box>
                <Text size="xs" c="dimmed">
                  Base Blocks
                </Text>
                <Text fw={500} size="sm">
                  Char: {charBaseBlock} / Scr: {screenBaseBlock}
                </Text>
              </Box>
            </Group>
          </Paper>
        </SimpleGrid>

        <Paper withBorder p="xs">
          <Group gap="xs">
            <ThemeIcon variant="light" color="orange">
              <IconPalette size={18} />
            </ThemeIcon>
            <Box>
              <Text size="xs" c="dimmed">
                Palette Mode
              </Text>
              <Text fw={500} size="sm">
                {colorMode === 0
                  ? "16 Colors / 16 Banks"
                  : "256 Colors / 1 Bank"}
              </Text>
            </Box>
          </Group>
        </Paper>

        <Stack p="xs">
          <Text size="xs" fw="bold" tt="uppercase" c="dimmed">
            Flags
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

export default BackgroundModal;
