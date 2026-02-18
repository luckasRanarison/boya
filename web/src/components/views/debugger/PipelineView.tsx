import { Group, Stack, Text } from "@mantine/core";
import { IconCaretRight, IconCaretRightFilled } from "@tabler/icons-react";
import { useRuntimeStore } from "@/stores/runtimeStore";
import MemoryLink from "@/components/common/MemoryLink";
import type { CpuState } from "@/hooks/useGba";
import { useDebuggerActions } from "@/stores/debuggerStore";
import { useEffect } from "react";
import { GBA } from "@/lib/gba";
import { useSearchParams } from "react-router";

function PipelineView({ cpu }: { cpu: CpuState }) {
  const [searchParams] = useSearchParams();
  const running = useRuntimeStore((state) => state.running);
  const { decode, pushStack, popStack } = useDebuggerActions();
  const pipeline = cpu.pipeline();

  useEffect(() => {
    if ((cpu.lr & ~1) === cpu.pc) {
      popStack();
    }

    if (GBA.startingSubroutine()) {
      pushStack({ caller: cpu.pc, return: cpu.pc + GBA.instructionSize() });
    }

    if (searchParams.get("mode") !== "code") {
      decode(2);
    }
  }, [cpu.lr, cpu.pc, searchParams, decode, pushStack, popStack]);

  return (
    <Group p="md" ff="monospace">
      {pipeline.length ? (
        <Stack w="100%">
          {pipeline.map((instr) => (
            <Group
              key={instr.address}
              c={instr.address === cpu.pc ? "indigo" : "gray"}
              w="100%"
            >
              {instr.address === cpu.pc ? (
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
