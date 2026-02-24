import type { RomData } from "@/stores/runtimeStore";
import { formatFileSize, formatHex } from "@/utils/format";
import { Stack, Text, Group, Badge, Divider, Paper } from "@mantine/core";

function RomCard(props: { data: RomData }) {
  const data = [
    { label: "Entry", value: formatHex(props.data.header.entryPoint) },
    { label: "Game Code", value: props.data.header.gameCode },
    { label: "Maker Code", value: props.data.header.makerCode },
    {
      label: "Checksum",
      value: formatHex(props.data.header.checksum, { width: 2 }),
    },
    { label: "Size", value: formatFileSize(props.data.metadata.size) },
  ];

  return (
    <Paper withBorder radius="md" p="md">
      <Stack gap="xs">
        <Group justify="space-between">
          <Text fw={700} ff="monospace" size="lg">
            {props.data.header.title}
          </Text>
          <Badge variant="outline" radius="sm">
            v{props.data.header.softwareVersion}
          </Badge>
        </Group>

        <Divider variant="dashed" />

        {data.map(({ label, value }) => (
          <Group key={label} justify="space-between">
            <Text size="xs" c="dimmed">
              {label}
            </Text>
            <Text size="xs">{value}</Text>
          </Group>
        ))}
      </Stack>
    </Paper>
  );
}

export default RomCard;
