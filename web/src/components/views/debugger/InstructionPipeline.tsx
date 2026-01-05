import { Group, Text } from "@mantine/core";
import { IconCaretRight } from "@tabler/icons-react";
import { instance } from "../../../lib/gba";

function InstructionPipeline() {
  const instruction = instance.currentInstruction();

  return (
    <Group ff="monospace">
      {instruction ? (
        <Group c="indigo">
          <IconCaretRight size={18} />
          <Text size="sm" fw={600}>
            {instruction}
          </Text>
        </Group>
      ) : (
        <Group c="red">
          <IconCaretRight size={18} />
          <Text size="sm">Pipeline is empty</Text>
        </Group>
      )}
    </Group>
  );
}

export default InstructionPipeline;
