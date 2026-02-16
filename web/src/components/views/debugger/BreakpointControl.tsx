import { useState } from "react";
import MemoryLink from "@/components/common/MemoryLink";
import { useBreakpoints, useDebuggerActions } from "@/stores/debuggerStore";
import { formatHex, parseHex } from "@/utils/format";
import { Stack, Group, ActionIcon, Button, Input } from "@mantine/core";
import { IconPlus, IconTrash } from "@tabler/icons-react";

type Edit = {
  index: number;
  value: string;
};

function BreakpointControl(props: { disabled: boolean }) {
  const [edit, setEdit] = useState<Edit | null>(null);
  const breakpoints = useBreakpoints();
  const { removeBreak, addBreak } = useDebuggerActions();

  const updateBreakpoint = (bp: number) => {
    if (!edit) return;

    const parsed = parseHex(edit.value);

    if (!Number.isNaN(parsed)) {
      removeBreak(bp);
      addBreak(parsed);
      setEdit(null);
    }
  };

  return (
    <Stack p="md">
      {breakpoints.size > 0 && (
        <Stack w="100%" gap="xs">
          {Array.from(breakpoints.values()).map((bp, i) => (
            <Group w="100%" key={i} align="center">
              <Input
                value={edit?.index === i ? edit.value : formatHex(bp)}
                onChange={(e) =>
                  setEdit({ index: i, value: e.currentTarget.value })
                }
                onFocus={() => setEdit({ index: i, value: formatHex(bp) })}
                onBlur={() => updateBreakpoint(bp)}
                onKeyDown={(e) => e.code === "Enter" && e.currentTarget.blur()}
                error={edit?.index === i && Number.isNaN(parseHex(edit.value))}
                flex={1}
                disabled={props.disabled}
              />
              <MemoryLink address={bp} disabled={props.disabled} />
              <ActionIcon
                color="red"
                variant="light"
                onClick={() => removeBreak(bp)}
                disabled={props.disabled}
              >
                <IconTrash size={16} />
              </ActionIcon>
            </Group>
          ))}
        </Stack>
      )}

      <Button
        leftSection={<IconPlus size={16} />}
        variant="light"
        onClick={() => addBreak(0)}
        disabled={props.disabled}
      >
        Add breakpoint
      </Button>
    </Stack>
  );
}

export default BreakpointControl;
