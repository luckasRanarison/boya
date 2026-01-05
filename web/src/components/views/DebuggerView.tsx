import { Divider, Group, SimpleGrid, Stack, Text } from "@mantine/core";
import { instance, psrFlags } from "../../lib/gba";
import { formatHex } from "../../utils";

function CpsrView(props: { value: number; label?: string }) {
  return (
    <Group>
      <Text>CPSR{props.label && `_${props.label}`}: </Text>
      <SimpleGrid cols={8}>
        <Text c={psrFlags.N && props.value !== 0 ? "indigo" : "gray"}>N</Text>
        <Text c={psrFlags.Z && props.value !== 0 ? "indigo" : "gray"}>Z</Text>
        <Text c={psrFlags.C && props.value !== 0 ? "indigo" : "gray"}>C</Text>
        <Text c={psrFlags.V && props.value !== 0 ? "indigo" : "gray"}>V</Text>
        <Text c={psrFlags.I && props.value !== 0 ? "indigo" : "gray"}>I</Text>
        <Text c={psrFlags.F && props.value !== 0 ? "indigo" : "gray"}>F</Text>
        <Text c={psrFlags.T && props.value !== 0 ? "indigo" : "gray"}>T</Text>
      </SimpleGrid>
    </Group>
  );
}

function RegisterList(props: {
  values: Uint32Array;
  label?: string;
  offset?: number;
}) {
  return (
    <Stack>
      <SimpleGrid cols={props.label ? 1 : 2}>
        {Array.from(props.values).map((r, i) => (
          <Group>
            <Text w={`${props.label ? props.label.length + 5 : 4}ch`}>
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

function RegisterBankView({
  regs,
  psr,
  label,
  offset,
  withDivider = true,
}: {
  regs: Uint32Array;
  psr: number;
  label?: string;
  offset?: number;
  withDivider?: boolean;
}) {
  return (
    <>
      <RegisterList values={regs} label={label} offset={offset} />
      <CpsrView value={psr} label={label} />
      {withDivider && <Divider />}
    </>
  );
}

function DebuggerView() {
  const psr = instance.getBankedPsr();

  return (
    <Stack
      w="100%"
      pt="xl"
      pb="10dvh"
      px="md"
      mah="90dvh"
      style={{ overflow: "scroll" }}
      ff="monospace"
    >
      <RegisterBankView
        regs={instance.getMainRegisters()}
        psr={instance.cpsr()}
        withDivider
      />

      <RegisterBankView
        label="fiq"
        regs={instance.getFiqRegisters()}
        psr={psr[0]}
        offset={7}
      />

      <RegisterBankView
        label="svc"
        regs={instance.getSvcRegisters()}
        psr={psr[1]}
        offset={13}
      />

      <RegisterBankView
        label="abt"
        regs={instance.getAbtRegisters()}
        psr={psr[2]}
        offset={13}
      />

      <RegisterBankView
        label="irq"
        regs={instance.getIrqRegisters()}
        psr={psr[3]}
        offset={13}
      />

      <RegisterBankView
        label="und"
        regs={instance.getUndRegisters()}
        psr={psr[4]}
        offset={13}
        withDivider={false}
      />
    </Stack>
  );
}

export default DebuggerView;
