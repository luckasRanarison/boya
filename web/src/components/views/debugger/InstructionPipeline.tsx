import { Group, Text } from "@mantine/core";
import { IconCaretRight } from "@tabler/icons-react";
import { instance } from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";

function InstructionPipeline() {
  const { instructionCache } = useDebuggerStore();
  const instruction = instructionCache[instance.pc()];

  return (
    <Group ff="monospace">
      {instruction ? (
        <Group c="indigo">
          <IconCaretRight size={18} />
          <Text size="sm" fw={600}>
            {instruction.value}
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
