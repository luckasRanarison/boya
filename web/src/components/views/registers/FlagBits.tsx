import { getFlagBits } from "@/utils/bitflag";
import { formatHex, getHexWidth } from "@/utils/format";
import { Table, Tooltip } from "@mantine/core";
import type { RegisterEntry } from "boya_wasm";

export function FlagBits({
  value,
  register,
}: {
  value: number;
  register: RegisterEntry;
}) {
  return (
    <Table w="0" fz="xs" withTableBorder withColumnBorders>
      <Table.Tbody>
        <Table.Tr>
          {register.flags.map(
            (f) =>
              register.size === "HWord" &&
              getFlagBits(value, f).map((b, i) => (
                <Tooltip
                  key={register.name + f.name + (f.start + i)}
                  label={f.name}
                >
                  {f.name === "unused" ? (
                    <Table.Td c="gray">-</Table.Td>
                  ) : (
                    <Table.Td>{b}</Table.Td>
                  )}
                </Tooltip>
              )),
          )}
          <Table.Td c="gray">
            {formatHex(value, { width: getHexWidth(register.size) })}
          </Table.Td>
        </Table.Tr>
      </Table.Tbody>
    </Table>
  );
}
