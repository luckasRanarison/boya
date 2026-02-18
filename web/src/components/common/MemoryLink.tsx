import { ActionIcon } from "@mantine/core";
import { IconExternalLink } from "@tabler/icons-react";
import type { MemoryViewMode } from "../views/memory/MemoryView";
import { formatHex } from "@/utils/format";
import { useGotoMemory } from "@/hooks/useGotoMemory";
import { useSearchParams } from "react-router";

function MemoryLink(props: {
  address: number;
  mode?: MemoryViewMode;
  disabled?: boolean;
  tooltip?: boolean;
}) {
  const gotoMemory = useGotoMemory();
  const [searchParams] = useSearchParams();
  const currentMode = searchParams.get("mode") as MemoryViewMode | undefined;

  const handleGoto = () => {
    gotoMemory({
      address: props.address,
      mode: currentMode ?? "code",
      hightlight: true,
    });
  };

  return (
    <ActionIcon
      variant="subtle"
      disabled={props.disabled}
      onClick={handleGoto}
      title={`Go to ${formatHex(props.address)}`}
    >
      <IconExternalLink size={18} />
    </ActionIcon>
  );
}

export default MemoryLink;
