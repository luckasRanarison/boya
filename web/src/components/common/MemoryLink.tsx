import { ActionIcon, Tooltip } from "@mantine/core";
import { IconExternalLink } from "@tabler/icons-react";
import type { MemoryViewMode } from "../views/memory/MemoryView";
import { formatHex } from "@/utils/format";
import { useGotoMemory } from "@/hooks/useGotoMemory";
import { useViewStore } from "@/stores/viewStore";

function MemoryLink(props: {
  address: number;
  mode?: MemoryViewMode;
  disabled?: boolean;
}) {
  const gotoMemory = useGotoMemory();
  const { view } = useViewStore();

  return (
    <Tooltip label={`Go to ${formatHex(props.address)}`}>
      <ActionIcon
        variant="subtle"
        disabled={props.disabled}
        onClick={() =>
          gotoMemory({
            address: props.address,
            mode:
              props.mode ??
              (view.name === "memory" ? view.sub?.metadata?.mode : undefined) ??
              "code",
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
