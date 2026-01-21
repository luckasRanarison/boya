import { ActionIcon, Group, Stack, Text } from "@mantine/core";
import {
  IconCaretRight,
  IconCaretRightFilled,
  IconExternalLink,
} from "@tabler/icons-react";
import { instance } from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { useView } from "@/stores/viewStore";

function InstructionPipeline() {
  const { instructionCache } = useDebuggerStore();
  const { gotoMemory } = useView();

  const pc = instance.execAddress();
  const next = pc + instance.instructionSize();

  const instructions = [
    { address: pc, value: instructionCache[pc]?.value },
    { address: next, value: instructionCache[next]?.value },
  ].filter((i) => i.value);

  return (
    <Group p="md" ff="monospace">
      {instructions.length ? (
        <Stack w="100%">
          {instructions.map((instr) => (
            <Group
              key={instr.address}
              c={instr.address === pc ? "indigo" : "gray"}
              w="100%"
            >
              {instr.address === pc ? (
                <IconCaretRightFilled size={18} />
              ) : (
                <IconCaretRight size={18} />
              )}

              <Text flex={1} size="sm" fw={600}>
                {instr.value}
              </Text>

              <ActionIcon
                variant="subtle"
                onClick={() => gotoMemory(instr.address, "code")}
              >
                <IconExternalLink size={18} />
              </ActionIcon>
            </Group>
          ))}
        </Stack>
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
