import { useRuntimeStore } from "@/stores/runtimeStore";
import { formatHex, getHexWidth } from "@/utils/format";
import { Accordion, Group, Stack, Text } from "@mantine/core";
import { FlagBits } from "./FlagBits";
import { FlagList } from "./FlagList";
import { memoryRegions } from "@/lib/gba";
import { useEffect, useState } from "react";
import Loader from "@/components/common/Loader";
import { useGba } from "@/hooks/useGba";

function IORegisterView(props: { style?: "simple" | "full" }) {
  const [loading, setLoading] = useState(true);
  const { running } = useRuntimeStore();
  const { memory } = useGba();

  const offset = memoryRegions.io.offset;
  const registers = memory.getIoRegisters();

  useEffect(() => {
    setTimeout(() => setLoading(false), 10);
  }, []);

  if (loading) {
    return <Loader />;
  }

  return (
    <Stack w="100%" p={0} ff="monospace">
      <Accordion>
        {registers.map((register) => {
          return (
            <Accordion.Item key={register.address} value={register.name}>
              <Accordion.Control disabled={running}>
                <Group justify="space-between" pr="md">
                  <Group gap="xl">
                    <Text c="indigo" fw={600}>
                      {formatHex(
                        props.style === "simple"
                          ? register.address + offset - offset
                          : register.address + offset,
                        { width: 3 },
                      )}
                    </Text>
                    <Text size={props.style === "simple" ? "sm" : "md"}>
                      {register.name}
                    </Text>
                  </Group>
                  {props.style === "simple" ? (
                    <Text c="gray">
                      {formatHex(register.value, {
                        width: getHexWidth(register.size),
                      })}
                    </Text>
                  ) : (
                    <Group style={{ overflow: "scroll" }}>
                      <FlagBits value={register.value} register={register} />
                    </Group>
                  )}
                </Group>
              </Accordion.Control>
              <Accordion.Panel>
                {register.flags.length ? (
                  <FlagList
                    value={register.value}
                    flags={register.flags}
                    showBits
                  />
                ) : (
                  <Group>
                    <Text size="sm">Value: </Text>
                    <Text size="sm" c="gray">
                      {register.value}
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
