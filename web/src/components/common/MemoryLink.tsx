import { ActionIcon, Tooltip } from "@mantine/core";
import { IconExternalLink } from "@tabler/icons-react";
import type { MemoryViewMode } from "../views/memory/MemoryView";
import { formatHex } from "@/utils/format";
import { useGotoMemory } from "@/hooks/useGotoMemory";

function MemoryLink(props: {
  address: number;
  mode?: MemoryViewMode;
  disabled?: boolean;
}) {
  const gotoMemory = useGotoMemory();

  return (
    <Tooltip label={`Go to ${formatHex(props.address)}`}>
      <ActionIcon
        variant="subtle"
        disabled={props.disabled}
        onClick={() =>
          gotoMemory({
            address: props.address,
            mode: props.mode ?? "code",
            hightlight: true,
          })
        }
      >
        <IconExternalLink size={18} />
      </ActionIcon>
    </Tooltip>
  );
}

export default MemoryLink;
