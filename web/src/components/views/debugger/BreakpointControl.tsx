import MemoryLink from "@/components/common/MemoryLink";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils/format";
import { Stack, TextInput, Group, ActionIcon, Button } from "@mantine/core";
import { IconPlus, IconTrash } from "@tabler/icons-react";
import { useState } from "react";

function BreakpointControl() {
  const { breakpoints } = useDebuggerStore();

  const [currentEdit, setCurrentEdit] = useState<{
    index: number;
    value: string;
  } | null>(null);

  const parse = (value: string) => {
    return value.startsWith("0x")
      ? parseInt(value.slice(2), 16)
      : parseInt(value, 10);
  };

  const updateBreakpoint = (bp: number) => {
    if (!currentEdit) return;

    const parsed = parse(currentEdit.value);

    if (Number.isNaN(parsed)) return;

    breakpoints.remove(bp);
    breakpoints.add(parsed);
    setCurrentEdit(null);
  };

  return (
    <Stack p="md">
      {breakpoints.entries.size > 0 && (
        <Group w="100%" gap="xs">
          {Array.from(breakpoints.entries.values()).map((bp, i) => (
            <Group w="100%" key={i} align="center">
              <TextInput
                value={
                  currentEdit?.index === i ? currentEdit.value : formatHex(bp)
                }
                onChange={(e) =>
                  setCurrentEdit({ index: i, value: e.currentTarget.value })
                }
                onFocus={() =>
                  setCurrentEdit({ index: i, value: formatHex(bp) })
                }
                onBlur={() => updateBreakpoint(bp)}
                onKeyDown={(e) => e.code === "Enter" && e.currentTarget.blur()}
                error={
                  currentEdit?.index === i &&
                  Number.isNaN(parse(currentEdit.value))
                }
                flex={1}
              />
              <MemoryLink address={bp} />
              <ActionIcon
                color="red"
                variant="light"
                onClick={() => breakpoints.remove(bp)}
              >
                <IconTrash size={16} />
              </ActionIcon>
            </Group>
          ))}
        </Group>
      )}

      <Button
        leftSection={<IconPlus size={16} />}
        variant="light"
        onClick={() => breakpoints.add(0)}
      >
        Add breakpoint
      </Button>
    </Stack>
  );
}

export default BreakpointControl;
