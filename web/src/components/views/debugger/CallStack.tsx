import MemoryLink from "@/components/common/MemoryLink";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils/format";
import { Flex, Group, Stack, Text } from "@mantine/core";

function CallStack(props: { disabled: boolean }) {
  const stack = useDebuggerStore((state) => state.callstack);

  return (
    <Stack p="md">
      {stack.length ? (
        [...stack].reverse().map((address, i) => (
          <Group key={i} justify="space-between" w="100%">
            <Text c="indigo">{formatHex(address.caller)}</Text>

            <MemoryLink address={address.caller} disabled={props.disabled} />
          </Group>
        ))
      ) : (
        <Flex justify="center">
          <Text size="sm" c="gray">
            (EMPTY CALLSTACK)
          </Text>
        </Flex>
      )}
    </Stack>
  );
}

export default CallStack;
