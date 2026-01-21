import { instance, memoryRegions } from "@/lib/gba";
import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils";
import { Accordion, Group, Stack, Text } from "@mantine/core";
import { type IOMap } from "boya_wasm";
import { useEffect, useMemo } from "react";
import { FlagBits } from "./FlagBits";
import { FlagList } from "./FlagList";

function IORegisterView(props: { style: "simple" | "full" }) {
  const { cycles } = useDebuggerStore();
  const registerMap = useMemo<IOMap>(() => instance.generateIOMap(), []);

  useEffect(() => {
    // re-render on cycle update
  }, [cycles]);

  return (
    <Stack w="100%" p={0} ff="monospace">
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
                <Group justify="space-between" pr="md">
                  <Group gap="xl">
                    <Text c="indigo" fw={600}>
                      {formatHex(
                        props.style === "simple"
                          ? address - memoryRegions.io.offset
                          : address,
                        { width: 3 },
                      )}
                    </Text>
                    <Text size={props.style === "simple" ? "sm" : "md"}>
                      {register.name}
                    </Text>
                  </Group>
                  {props.style === "simple" ? (
                    <Text c="gray">
                      {formatHex(value, {
                        width: register.size === "HWord" ? 4 : 8,
                      })}
                    </Text>
                  ) : (
                    <Group style={{ overflow: "scroll" }}>
                      <FlagBits value={value} register={register} />
                    </Group>
                  )}
                </Group>
              </Accordion.Control>
              <Accordion.Panel>
                {register.flags.length ? (
                  <FlagList value={value} flags={register.flags} />
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

export default IORegisterView;
