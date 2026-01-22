import { formatHex } from "@/utils/format";
import { Card, SimpleGrid, Tooltip } from "@mantine/core";

function ColorView(props: { value: number[]; columns: number }) {
  return (
    <SimpleGrid style={{ width: "fit" }} cols={props.columns} spacing={0}>
      {props.value.map((color, i) => {
        const hex = formatHex(color, { prefix: "#", width: 6 });
        return (
          <Tooltip key={i} label={hex}>
            <Card p="xs" radius={0} bg={hex} withBorder />
          </Tooltip>
        );
      })}
    </SimpleGrid>
  );
}

export default ColorView;
