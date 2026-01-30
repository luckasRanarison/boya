import type { MemoryViewMode } from "@/components/views/memory/MemoryView";
import notifications from "@/lib/notifications";
import { useViewActions } from "@/stores/viewStore";
import { formatHex } from "@/utils/format";
import { useGba } from "./useGba";
import type { MemoryRegionName } from "@/lib/gba";

export function useGotoMemory() {
  const { memory } = useGba();
  const { setView } = useViewActions();

  return (params: {
    address: number;
    mode: MemoryViewMode;
    hightlight?: boolean;
  }) => {
    const hex = formatHex(params.address);

    const findElement = (depth = 0) => {
      if (depth > 3) {
        return notifications.error(`Invalid jump address: ${hex}`);
      }

      const elem = document.getElementById(hex);

      if (!elem) {
        const link = document.createElement("a");
        link.href = `#${hex}`;
        link.click();
        link.remove();

        return setTimeout(() => findElement(depth + 1), 100); // add timeout to avoid busy loop
      }

      elem.scrollIntoView({ block: "center", behavior: "smooth" });

      if (params?.hightlight && !elem.classList.contains(".goto-highlight")) {
        elem.classList.add("goto-highlight");

        setTimeout(() => {
          elem.classList.remove("goto-highlight");
        }, 2000);
      }
    };

    const region = Object.keys(memory.regions).find((name) => {
      const region = memory.getRegion(name as MemoryRegionName);

      return (
        params.address >= region.offset &&
        params.address < region.offset + region.getLength()
      );
    });

    if (!region) {
      return notifications.error(`Invalid jump address: ${hex}`);
    }

    setView({
      name: "memory",
      sub: {
        name: region,
        metadata: {
          mode: params.mode ?? "code",
          jump: { address: params.address },
        },
      },
    });

    findElement();
  };
}
