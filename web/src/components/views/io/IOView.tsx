import { instance, memoryRegions } from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils";
import { Group, SimpleGrid, Text } from "@mantine/core";
import { type IOMap } from "boya_wasm";
import { useEffect, useMemo } from "react";

function IOView() {
  const registerMap = useMemo<IOMap>(() => instance.generateIOMap(), []);
  const { cycles } = useDebuggerStore();

  useEffect(() => {
    // re-render on cycle update
  }, [cycles]);

  return (
    <SimpleGrid p="xl">
      {registerMap.map((register) => {
        const address = memoryRegions.io.offset + register.address;

        return (
          <Group key={address}>
            <Text>{formatHex(address)}</Text>
            <Text>{register.name}</Text>
            <Text>
              {register.size === "HWord"
                ? formatHex(instance.readHWord(address), { width: 2 })
                : formatHex(instance.readWord(address), { width: 4 })}
            </Text>
          </Group>
        );
      })}
    </SimpleGrid>
  );
}

export default IOView;
