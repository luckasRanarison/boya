import { Group, SimpleGrid, Stack, Text } from "@mantine/core";
import { instance } from "@/lib/gba";
import { formatHex } from "@/utils";
import ColorView from "./ColorView";

type ByteLine = {
  address: number;
  columns: number[];
  right:
    | { type: "color"; value: number[] }
    | { type: "ascii"; value: number[] };
};

function HexView(props: {
  pageData: Uint8Array;
  baseAddress: number;
  pageStart: number;
  columns: number;
  rightSection?: "color" | "ascii";
}) {
  const colors = props.rightSection === "color" && instance.colorPalette();

  const generateLines = () => {
    const lines: ByteLine[] = [];

    for (let i = 0; i < props.pageData.length; i += props.columns) {
      const row = props.pageData.slice(i, i + props.columns);
      const bytes = Array.from(row);

      lines.push({
        address: props.baseAddress + props.pageStart + i,
        columns: bytes,
        right: colors
          ? {
              type: "color",
              value: Array.from(colors.slice(i / 2, i / 2 + 8)),
            }
          : { type: "ascii", value: bytes },
      });
    }

    return lines;
  };

  const lines = generateLines();

  return (
    <Stack p="xl" w="100%" ff={"monospace"} align="center">
      {lines.map((line) => (
        <Group key={line.address} w="100%" justify="space-between">
          <Text c="indigo" fw={600}>
            {formatHex(line.address)}:
          </Text>
          <SimpleGrid
            spacing="md"
            cols={{ base: 8, sm: 16 }}
            w={{ base: "100%", sm: "auto" }}
          >
            {line.columns.map((byte, idx) => (
              <Text key={line.address + idx} c="gray" ta="center">
                {byte.toString(16).padStart(2, "0")}
              </Text>
            ))}
          </SimpleGrid>

          {line.right.type === "color" && (
            <ColorView value={line.right.value} columns={8} />
          )}

          {line.right.type === "ascii" && (
            <Text w={`${props.columns}ch`} c="indigo.4">
              {line.right.value
                .map((b) =>
                  b >= 32 && b <= 126 ? String.fromCharCode(b) : ".",
                )
                .join("")}
            </Text>
          )}
        </Group>
      ))}
    </Stack>
  );
}

export default HexView;
