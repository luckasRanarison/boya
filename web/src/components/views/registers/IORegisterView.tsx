import { useDebuggerStore } from "@/stores/debuggerStore";
import { formatHex } from "@/utils/format";
import { Accordion, Group, Stack, Text } from "@mantine/core";
import { FlagBits } from "./FlagBits";
import { FlagList } from "./FlagList";
import { memoryRegions, type IORegister } from "@/lib/gba";

function IORegisterView(props: {
  value: IORegister;
  style: "simple" | "full";
}) {
  const { running } = useDebuggerStore();
  const offset = memoryRegions.io.offset;

  return (
    <Stack w="100%" p={0} ff="monospace">
      <Accordion>
        {props.value.map((register) => {
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
                        width: register.size === "HWord" ? 4 : 8,
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
                  <FlagList value={register.value} flags={register.flags} />
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
