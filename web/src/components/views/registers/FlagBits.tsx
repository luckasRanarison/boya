import { getFlagValue } from "@/utils/bitflag";
import { formatHex } from "@/utils/format";
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
          {register.flags.map((f) =>
            getFlagValue(value, f)
              .toString(10)
              .padStart(f.length, "0")
              .split("")
              .map((b, i) => (
                <Tooltip
                  key={register.name + f.name + (f.start + i)}
                  label={f.name}
                >
                  {f.name === "unused" ? (
                    <Table.Td c="gray">-</Table.Td>
                  ) : (
                    <Table.Td
                      style={{
                        borderColor:
                          f.length > 1 && i !== f.length - 1
                            ? "transparent"
                            : undefined,
                      }}
                    >
                      {b}
                    </Table.Td>
                  )}
                </Tooltip>
              )),
          )}
          <Table.Td c="gray">
            {formatHex(value, { width: register.size === "HWord" ? 4 : 8 })}
          </Table.Td>
        </Table.Tr>
      </Table.Tbody>
    </Table>
  );
}
