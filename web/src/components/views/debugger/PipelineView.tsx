import { Group, Stack, Text } from "@mantine/core";
import { IconCaretRight, IconCaretRightFilled } from "@tabler/icons-react";
import { useRuntimeStore } from "@/stores/runtimeStore";
import MemoryLink from "@/components/common/MemoryLink";
import type { InstructionPipeline } from "@/hooks/useGba";

function PipelineView(props: { base: number; pipeline: InstructionPipeline }) {
  const { running } = useRuntimeStore();

  return (
    <Group p="md" ff="monospace">
      {props.pipeline.length ? (
        <Stack w="100%">
          {props.pipeline.map((instr) => (
            <Group
              key={instr.address}
              c={instr.address === props.base ? "indigo" : "gray"}
              w="100%"
            >
              {instr.address === props.base ? (
                <IconCaretRightFilled size={18} />
              ) : (
                <IconCaretRight size={18} />
              )}

              <Text flex={1} size="sm" fw={600}>
                {instr.value}
              </Text>

              <MemoryLink address={instr.address} disabled={running} />
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

export default PipelineView;
