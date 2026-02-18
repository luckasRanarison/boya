import type { MemoryViewMode } from "@/components/views/memory/MemoryView";
import notifications from "@/lib/notifications";
import { formatHex } from "@/utils/format";
import {
  getMemoryRegion,
  memoryRegions,
  type MemoryRegionName,
} from "@/lib/gba";
import { useNavigate } from "react-router";

export function useGotoMemory() {
  const navigate = useNavigate();

  return (params: {
    address: number;
    mode: MemoryViewMode;
    hightlight?: boolean;
  }) => {
    const hex = formatHex(params.address);
    const searchId = Math.random().toString(36).substring(7); // use rng to force re-render

    const findElement = (depth = 0) => {
      if (depth > 20) {
        return notifications.error(`Invalid jump address: ${hex}`);
      }

      const elem = document.getElementById(hex);

      if (!elem) {
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

    const region = Object.keys(memoryRegions).find((name) => {
      const region = getMemoryRegion(name as MemoryRegionName);

      return (
        params.address >= region.offset &&
        params.address < region.offset + region.getLength()
      );
    });

    if (!region) {
      return notifications.error(`Invalid jump address: ${hex}`);
    }

    navigate(
      {
        pathname: `/memory/${region.toLowerCase()}`,
        search: `?mode=${params.mode ?? "code"}&jump=${`${params.address}.${searchId}`}`,
      },
      { replace: true },
    );

    findElement();
  };
}
