import { useDebuggerStore } from "@/stores/debuggerStore";
import { useView } from "@/stores/viewStore";
import { formatHex } from "@/utils";
import { Stack, TextInput, Group, ActionIcon, Button } from "@mantine/core";
import { IconExternalLink, IconPlus, IconTrash } from "@tabler/icons-react";
import { useState } from "react";

function BreakpointControl() {
  const { breakpoints, setBreakpoints } = useDebuggerStore();
  const { gotoMemory } = useView();

  const [currentEdit, setCurrentEdit] = useState<{
    index: number;
    value: string;
  } | null>(null);

  const parse = (value: string) => {
    return value.startsWith("0x")
      ? parseInt(value.slice(2), 16)
      : parseInt(value, 10);
  };

  const updateBreakpoint = () => {
    if (!currentEdit) return;

    const parsed = parse(currentEdit.value);

    if (Number.isNaN(parsed)) return;

    const next = [...breakpoints];
    next[currentEdit.index] = parsed;
    setBreakpoints(next);
    setCurrentEdit(null);
  };

  const removeBreakpoint = (index: number) => {
    setBreakpoints(breakpoints.filter((_, i) => i !== index));
  };

  const addBreakpoint = () => {
    setBreakpoints([...breakpoints, 0]);
  };

  return (
    <Stack p="md">
      {breakpoints.length > 0 && (
        <Group w="100%" gap="xs">
          {breakpoints.map((bp, i) => (
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
                onBlur={() => updateBreakpoint()}
                onKeyDown={(e) => e.code === "Enter" && e.currentTarget.blur()}
                error={
                  currentEdit?.index === i &&
                  Number.isNaN(parse(currentEdit.value))
                }
                flex={1}
              />
              <ActionIcon
                variant="subtle"
                onClick={() => gotoMemory(bp, "code")}
              >
                <IconExternalLink size={18} />
              </ActionIcon>
              <ActionIcon
                color="red"
                variant="light"
                onClick={() => removeBreakpoint(i)}
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
        onClick={addBreakpoint}
      >
        Add breakpoint
      </Button>
    </Stack>
  );
}

export default BreakpointControl;
