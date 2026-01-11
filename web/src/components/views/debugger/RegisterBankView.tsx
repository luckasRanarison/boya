import { Text, Group, SimpleGrid, Stack, Divider } from "@mantine/core";
import { getRegistersBank, psrFlags } from "@/lib/gba";
import { formatHex } from "../../../utils";

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
    <Group>
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
}) {
  return (
    <Stack>
      <SimpleGrid cols={props.label ? 1 : 2}>
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

function RegisterBankView() {
  const banks = getRegistersBank();

  return (
    <Stack ff="monospace">
      {banks.map((b, i) => (
        <Stack key={b.label ?? "main"}>
          <RegisterView
            values={b.registers}
            label={b.label}
            offset={b.offset}
          />
          <CpsrView value={b.psr} label={b.label} />
          {i !== banks.length - 1 && <Divider />}
        </Stack>
      ))}
    </Stack>
  );
}

export default RegisterBankView;
