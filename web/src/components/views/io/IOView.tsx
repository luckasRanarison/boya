import { getFlagValue } from "@/lib/bitflap";
import { instance, memoryRegions } from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils";
import { Accordion, Group, Stack, Text } from "@mantine/core";
import { type IOMap } from "boya_wasm";
import { useEffect, useMemo } from "react";

function IOView() {
  const { cycles } = useDebuggerStore();
  const registerMap = useMemo<IOMap>(() => instance.generateIOMap(), []);

  useEffect(() => {
    // re-render on cycle update
  }, [cycles]);

  return (
    <Stack w="100%" p="xl" ff="monospace">
      <Accordion>
        {registerMap.map((register) => {
          const address = memoryRegions.io.offset + register.address;
          const value =
            register.size === "HWord"
              ? instance.readHWord(address)
              : instance.readWord(address);

          return (
            <Accordion.Item key={address} value={register.name}>
              <Accordion.Control>
                <Group justify="space-between">
                  <Group gap="xl">
                    <Text c="indigo" fw={600}>
                      {formatHex(address)}
                    </Text>
                    <Text>{register.name}</Text>
                  </Group>
                  <Text mr="md" c="gray">
                    {register.size === "HWord"
                      ? formatHex(value, { width: 2 })
                      : formatHex(value, { width: 4 })}
                  </Text>
                </Group>
              </Accordion.Control>
              <Accordion.Panel>
                {register.flags.length ? (
                  <Stack>
                    {register.flags.map((f) => (
                      <Group>
                        <Text c="indigo" size="sm" w="7ch" ta="center">
                          [
                          {f.length === 1
                            ? f.start
                            : `${f.start}-${f.start + f.length - 1}`}
                          ]
                        </Text>
                        <Group flex={1}>
                          <Text size="sm">{f.name}:</Text>
                          <Text size="sm" c="gray">
                            {f.mappings
                              ? f.mappings[getFlagValue(value, f)]
                              : getFlagValue(value, f)}
                          </Text>
                        </Group>
                      </Group>
                    ))}
                  </Stack>
                ) : (
                  <Group>
                    <Text size="sm">Value: </Text>
                    <Text size="sm" c="gray">
                      {value}
                    </Text>
                  </Group>
                )}
              </Accordion.Panel>
            </Accordion.Item>
          );
        })}
      </Accordion>
    </Stack>
  );
}

export default IOView;
