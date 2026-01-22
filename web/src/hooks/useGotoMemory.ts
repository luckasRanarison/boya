import type { MemoryViewMode } from "@/components/views/memory/MemoryView";
import { memoryRegions } from "@/lib/gba";
import notifications from "@/lib/notifications";
import { useView } from "@/stores/viewStore";
import { formatHex } from "@/utils/format";

export function useGotoMemory() {
  const { setView } = useView();

  return (params: {
    address: number;
    mode: MemoryViewMode;
    hightlight?: boolean;
  }) => {
    const hex = formatHex(params.address);

    const findElement = (depth = 0) => {
      if (depth > 1) {
        return notifications.error(`Invalid jump address: ${hex}`);
      }

      const elem = document.getElementById(hex);

      if (!elem) {
        const link = document.createElement("a");
        link.href = `#${hex}`;
        link.click();
        link.remove();

        return setTimeout(() => findElement(depth + 1), 200); // add timeout to avoid busy loop
      }

      elem.scrollIntoView({ block: "center", behavior: "smooth" });

      if (params?.hightlight && !elem.classList.contains(".goto-highlight")) {
        elem.classList.add("goto-highlight");

        setTimeout(() => {
          elem.classList.remove("goto-highlight");
        }, 2000);
      }
    };

    const region = Object.entries(memoryRegions).find(([, data]) => {
      return params.address < data.offset + data.length;
    });

    if (region) {
      setView({
        name: "memory",
        sub: {
          name: region[0],
          metadata: {
            mode: params.mode ?? "code",
            jump: { address: params.address },
          },
        },
      });

      findElement();
    }
  };
}
