import type { MemoryViewMode } from "@/components/views/memory/MemoryView";
import { memoryRegions } from "@/lib/gba";
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

    const findElement = () => {
      const elem = document.getElementById(hex);

      if (!elem) {
        const link = document.createElement("a");
        link.href = `#${hex}`;
        link.click();
        link.remove();

        return setTimeout(findElement, 200); // add timeout to avoid busy loop
      }

      if (elem.classList.contains(".goto-highlight") || !params?.hightlight)
        return;

      elem.scrollIntoView({ block: "center", behavior: "smooth" });
      elem.classList.add("goto-highlight");

      setTimeout(() => {
        elem.classList.remove("goto-highlight");
      }, 2000);
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
            address: params.address,
          },
        },
      });

      findElement();
    }
  };
}
