import { Text, Group, SimpleGrid, Stack, Accordion } from "@mantine/core";
import { getRegistersBank, psrFlags } from "@/lib/gba";
import { formatHex } from "@/utils/format";

function CpsrFlag(props: { label: string; value: number; flag: number }) {
  return (props.flag & props.value) !== 0 ? (
    <Text size="sm" c="indigo" fw={600}>
      {props.label}
    </Text>
  ) : (
    <Text size="sm" c="gray">
      {props.label}
    </Text>
  );
}

function CpsrView(props: { value: number; label?: string }) {
  return (
    <Group px="md">
      <Text size="sm">CPSR{props.label && `_${props.label}`}: </Text>
      <SimpleGrid cols={8}>
        <CpsrFlag label="N" value={props.value} flag={psrFlags.N} />
        <CpsrFlag label="Z" value={props.value} flag={psrFlags.Z} />
        <CpsrFlag label="C" value={props.value} flag={psrFlags.C} />
        <CpsrFlag label="V" value={props.value} flag={psrFlags.V} />
        <CpsrFlag label="I" value={props.value} flag={psrFlags.I} />
        <CpsrFlag label="F" value={props.value} flag={psrFlags.F} />
        <CpsrFlag label="T" value={props.value} flag={psrFlags.T} />
      </SimpleGrid>
    </Group>
  );
}

function RegisterView(props: {
  values: Uint32Array;
  label?: string;
  offset?: number;
  style: "simple" | "full";
}) {
  return (
    <Stack px="md">
      <SimpleGrid
        cols={
          props.style === "simple"
            ? props.label
              ? 1
              : 2
            : { base: 2, sm: 3, md: 4, xl: 5 }
        }
      >
        {Array.from(props.values).map((r, i) => (
          <Group key={i}>
            <Text size="sm" w={`${props.label ? props.label.length + 5 : 4}ch`}>
              R{i + (props.offset ?? 0)}
              {props.label && `_${props.label}`}:
            </Text>
            <Text c="gray"> {formatHex(r)}</Text>
          </Group>
        ))}
      </SimpleGrid>
    </Stack>
  );
}

function RegisterBankView(props: { style: "simple" | "full" }) {
  const banks = getRegistersBank();

  return (
    <Stack w="100%" ff="monospace">
      <Accordion multiple defaultValue={["main"]}>
        {banks.map((b, i) => (
          <Accordion.Item value={b.label ?? "main"} key={i}>
            <Accordion.Control>{b.label ?? "main"}</Accordion.Control>
            <Accordion.Panel
              styles={{
                content: {
                  padding: 0,
                },
              }}
            >
              <Stack py="md" gap="xl">
                <RegisterView
                  values={b.registers}
                  label={b.label}
                  offset={b.offset}
                  style={props.style}
                />
                <CpsrView value={b.psr} label={b.label} />
              </Stack>
            </Accordion.Panel>
          </Accordion.Item>
        ))}
      </Accordion>
    </Stack>
  );
}

export default RegisterBankView;
